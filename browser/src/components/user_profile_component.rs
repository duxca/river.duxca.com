use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserProfileProps {
    pub name: String,
    pub email: String,
    pub profile_image_url: String,
}

#[function_component(UserProfileComponent)]
pub fn user_profile_component(props: &UserProfileProps) -> Html {
    html! {
        <div class="user-profile">
            <img class="user-profile__image" src={props.profile_image_url.clone()} alt="Profile Image" />
            <div class="user-profile__details">
                <p class="user-profile__name">{ &props.name }</p>
                <p class="user-profile__email">{ &props.email }</p>
            </div>
        </div>
    }
}