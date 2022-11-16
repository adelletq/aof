use yew::prelude::*;
use yew_router::prelude::*;

use yew::html::Scope;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/blog/:id")]
    Blog { id: String },
    #[at("/schedule")]
    Schedule,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

enum Msg {
    ToggleNavBar,
    PageUpdated
}

#[function_component(About)]
fn about() -> Html {
    html! {
            <div>
                <p>{"About"}</p>
            </div>
    }
}

struct App {
    navbar_visible: bool,
//     _listener: HistoryListener,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_visible: false,
//             _listener: listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavBar => {
                self.navbar_visible = !self.navbar_visible;
                true
            }
            Msg::PageUpdated => {
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {    
        html! {
            <div>
                <BrowserRouter>
                <header>
                <nav>
                <Link<Route> to={Route::Home}>{ "Art of Feeling" }</Link<Route>>
                <Link<Route> to={Route::About}>{ "Despre noi" }</Link<Route>>
                <Link<Route> to={Route::Blog { id: "1".to_string() }}>{ "Blog" }</Link<Route>>
                <Link<Route> to={Route::Schedule}>{ "Programări" }</Link<Route>>
                <Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>>
                </nav>
                </header>
                <Switch<Route> render={Switch::render(switch)} />
                <footer>
                {"Copyright 2022 Art of Feeling"}
                </footer>
                </BrowserRouter>
            </div>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <>
            <h1>{ "Arta de a simți" }</h1>
            <p>{ "Te ajutăm să deprinzi arta de a trăi armonios alături de ceilalți; atât în viața personală, cât și în cea profesională și anume arta de a simți cu adevărat viața!" }</p>
            <h2>{ "Arta de a simți" }</h2>
            <p>{ "Te ajutăm să deprinzi arta de a trăi armonios alături de ceilalți; atât în viața personală, cât și în cea profesională și anume arta de a simți cu adevărat viața!" }</p>
            <h3>{ "Arta de a simți" }</h3>
            <p>{ "Te ajutăm să deprinzi arta de a trăi armonios alături de ceilalți; atât în viața personală, cât și în cea profesională și anume arta de a simți cu adevărat viața!" }</p>
            <h4>{ "Arta de a simți" }</h4>
            <p>{ "Te ajutăm să deprinzi arta de a trăi armonios alături de ceilalți; atât în viața personală, cât și în cea profesională și anume arta de a simți cu adevărat viața!" }</p>
            </>
        },
        Route::About => html! { <><h1>{ "Adela Margin" }</h1>
            <About />
            </>
        },
        Route::Blog { id } => html! {<p>{format!("You are looking at Blog {}", id)}</p>},
        Route::Schedule => html! { <><h1>{ "Programări" }</h1>
            <About />
            </>
        },
        Route::Contact => html! { <><h1>{ "Contact" }</h1>
            <About />
            </>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::start_app::<App>();
}
