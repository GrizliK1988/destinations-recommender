use graphql_client::{ GraphQLQuery, Response as GraphQLResponse };
use yew::{html, Component, ComponentLink, Html, Properties};
use crate::photos::UserPreference;
use yew::services::fetch::{ Request, FetchTask };
use yew::services::FetchService;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/recommendation_query.graphql",
    response_derives = "Debug"
)]
pub struct RecommendedDestinations;

pub struct Recommendations {
    user_preferences: Vec<UserPreference>,
    task: Option<FetchTask>,
    recommendations: Vec<recommended_destinations::RecommendedDestinationsRecommendation>,
    fetch: FetchService,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct RecommendationsProperties {
    pub user_preferences: Vec<UserPreference>,
}

pub enum Msg {
    RecommendationsFetched(String),
    RecommendationsFetchFailed,
}

impl Component for Recommendations {
    type Message = Msg;
    type Properties = RecommendationsProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut component = Recommendations {
            user_preferences: props.user_preferences,
            task: None,
            recommendations: Vec::new(),
            fetch: FetchService::new(),
            link,
        };

        component.fetch_recommendations();

        component
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        let render = {
            match msg {
                Msg::RecommendationsFetched(recommendations) => {
                    let recommendations: GraphQLResponse<recommended_destinations::ResponseData> =
                        serde_json::from_str(recommendations.as_str())
                            .expect("Deserialization failed");

                    self.task = None;
                    self.recommendations = recommendations.data.unwrap().recommendation;

                    true
                }
                Msg::RecommendationsFetchFailed => {
                    false
                }
            }
        };

        render
    }

    fn view(&self) -> Html {
        let view = {
            match self.task {
                None => {
                    html! {
                      <ul class="list">
                        { for self.recommendations.iter().map(| recommendation | self.render_recommendation(recommendation)) }
                      </ul>
                    }
                }
                Some(_) => {
                    html! {
                      <div>
                        { "loading" }
                      </div>
                    }
                }
            }
        };

        view
    }
}

impl Recommendations {
    fn render_recommendation(&self, recommendation: &recommended_destinations::RecommendedDestinationsRecommendation) -> Html {
        html! {
            <li class="list-item">
                { recommendation.destination.to_owned() }{":"} {recommendation.score}
            </li>
        }
    }

    fn fetch_recommendations(&mut self) {
        let mapped_preferences = self.user_preferences
            .clone()
            .into_iter()
            .map(| el | {
                recommended_destinations::UserPreference {
                    like: el.like,
                    marker: el.marker,
                }
            })
            .collect();

        let query = RecommendedDestinations::build_query(recommended_destinations::Variables {
            user_preferences: mapped_preferences,
        });

        let query_content = serde_json::to_value(query).unwrap().to_string();

        let recommendations_request = Request
            ::post("http://localhost:8001/graphql")
                .header("Content-Type", "application/json")
                .body(Ok(query_content))
                .unwrap();

        let task = self.fetch.fetch(
            recommendations_request,
            self.link.callback(| response: http::response::Response<Result<String, failure::Error>> | {
                match response.body() {
                    Ok(recommendations_response) => {
                        Msg::RecommendationsFetched(recommendations_response.clone())
                    }
                    Err(_) => {
                        Msg::RecommendationsFetchFailed
                    }
                }
            })
        );
        self.task = Some(task);
    }
}
