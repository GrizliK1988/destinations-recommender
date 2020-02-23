use yew::{html, Component, Html, ComponentLink, ShouldRender};
use yew::services::{ ConsoleService, FetchService };
use graphql_client::{ GraphQLQuery, Response as GraphQLResponse };
use yew::services::fetch::{ Request, FetchTask };
use failure;
use crate::recommendations::Recommendations;

#[derive(Clone)]
pub struct UserPreference {
    pub marker: String,
    pub like: bool,
}

pub struct Photos {
    console: ConsoleService,
    fetch: FetchService,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    photos: Vec<poi_photos::PoiPhotosPhotos>,
    active_image_index: usize,
    pub user_preferences: Vec<UserPreference>,
}

pub enum Msg {
    Like,
    Dislike,
    PhotosFetched(String),
    PhotosFetchFailed,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/photos_query.graphql",
    response_derives = "Debug"
)]
pub struct PoiPhotos;

impl Component for Photos {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut layout = Photos {
            console: ConsoleService::new(),
            fetch: FetchService::new(),
            link,
            task: None,
            photos: Vec::new(),
            active_image_index: 0,
            user_preferences: Vec::new(),
        };

        layout.fetch_photos();

        layout
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let render = {
            match msg {
                Msg::Like => {
                    self.handle_user_reaction(true)
                }
                Msg::Dislike => {
                    self.handle_user_reaction(false)
                }
                Msg::PhotosFetched(photos_content) => {
                    let photos: GraphQLResponse<poi_photos::ResponseData> =
                        serde_json::from_str(photos_content.as_str())
                            .expect("Deserialization failed");

                    self.task = None;
                    self.photos = photos.data.unwrap().photos;
                    self.active_image_index = 0;
                    self.user_preferences = Vec::with_capacity(self.photos.len());

                    true
                }
                Msg::PhotosFetchFailed => {
                    self.task = None;

                    self.console.error("Photos fetch failed");

                    true
                }
            }
        };

        render
    }

    fn view(&self) -> Html {
        let photo = {
            match self.photos.get(self.active_image_index) {
                Some(first_photo) => String::from("/photos/".to_owned() + first_photo.file.as_str()),
                None => String::from("https://bulma.io/images/placeholders/480x640.png")
            }
        };

        let view = if self.active_image_index == self.photos.len() && self.photos.len() > 0 {
          html! {
            <Recommendations user_preferences={ self.user_preferences.clone() } />
          }
        } else {
            html! {
              <div>
                <div class="columns is-mobile is-gapless">
                    <div class="column">
                        <section class="hero">
                          <div class="hero-body">
                              <div class="container">
                                  <h1 class="title">
                                      { "Like your trip" }
                                  </h1>
                              </div>
                          </div>
                        </section>
                        <div class="card">
                          <div class="card-image">
                            <figure class="image">
                              <img
                                src={ photo }
                              />
                            </figure>
                          </div>
                        </div>
                    </div>
                </div>
                <nav class="navbar is-fixed-bottom">
                  <div class="columns is-mobile is-gapless">
                    <div class="column">
                      <a
                        class="button is-large is-fullwidth is-danger"
                        onclick=self.link.callback(|_| Msg::Dislike)
                      >
                        <span class="icon">
                          <i class="fas fa-thumbs-down fa-lg"></i>
                        </span>
                        <span>{ "Move on" }</span>
                      </a>
                    </div>
                    <div class="column">
                      <a
                        class="button is-large is-fullwidth is-success"
                        onclick=self.link.callback(|_| Msg::Like)
                      >
                        <span class="icon">
                          <i class="fas fa-thumbs-up fa-lg"></i>
                        </span>
                        <span>{ "Like it" }</span>
                      </a>
                    </div>
                  </div>
                </nav>
              </div>
            }
        };

        view
    }
}

impl Photos {
    fn handle_user_reaction(&mut self, like: bool) -> bool {
        if self.active_image_index < self.photos.len() {
            let photo = self.photos.get(self.active_image_index).unwrap();
            self.active_image_index += 1;

            self.user_preferences.push(UserPreference {
                marker: photo.marker.clone(),
                like,
            });

            true
        } else {
            false
        }
    }

    fn fetch_photos(&mut self) -> () {
        let query = PoiPhotos::build_query(poi_photos::Variables {
            count_per_category: 3,
        });

        let query_content = serde_json::to_value(query).unwrap().to_string();

        let photos_request = Request
            ::post("http://localhost:8001/graphql")
            .header("Content-Type", "application/json")
            .body(Ok(query_content))
            .unwrap();

        let task = self.fetch.fetch(
            photos_request,
            self.link.callback(| response: http::response::Response<Result<String, failure::Error>> | {
                match response.body() {
                    Ok(photos_response) => {
                        Msg::PhotosFetched(photos_response.clone())
                    }
                    Err(_) => {
                        Msg::PhotosFetchFailed
                    }
                }
            })
        );
        self.task = Some(task);

        ()
    }
}
