use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use chrono::Datelike;

use aof;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/author/:id")]
    Author { id: i32 },
    #[at("/services")]
    Services,
    #[at("/blog/:id")]
    Blog { id: i32 },
    #[at("/schedule")]
    Schedule,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

enum FetchState {
    Fetching,
    Success(aof::BlogEntry),
    Failed(gloo_net::Error)
}

enum Msg {
    SetBlogFetchState (FetchState)
}

struct Blog {
    entry: FetchState,
}

async fn fetch_blog(id: i32) -> Result<aof::BlogEntry, gloo_net::Error> {
    let url = format!("/api/blog/{}", id);
    let resp = Request::get(url.as_str()).header("Content-Type", "application/json").send().await.unwrap();
    resp.json::<aof::BlogEntry>().await
}

#[derive(Clone, PartialEq, Properties)]
struct BlogProps {
    id: i32,
}

impl Component for Blog {
    type Message = Msg;
    type Properties = BlogProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.clone();
        ctx.link().send_future(async move {
            match fetch_blog(id).await {
                Ok(blog) => Msg::SetBlogFetchState(FetchState::Success(blog)),
                Err(err) => Msg::SetBlogFetchState(FetchState::Failed(err))
            }
        });
        Self { entry: FetchState::Fetching }
    }

//     fn changed(&mut self, _ctx: &Context<Self>) -> bool {
//         self.entry = Some(aof::BlogEntry {
//             id: 1,
//             title: "Title".to_string(),
//             image_url: { "/static/aof1.jpg".to_string() },
//             author: {
//                 aof::Author {
//                     id: 1,
//                     name: "Adela Margin".to_string(),
//                 }
//             },
//             when: { Utc::now() },
//             description: "description".to_string(),
//             contents: vec![],
//         });
//         true
//     }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetBlogFetchState(fetch_state) => {
                self.entry = fetch_state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { entry } = self;
        match entry {
            FetchState::Fetching => {
                html! { <main><div class="fetching">{format!("Fetching blog id {}", _ctx.props().id)}</div></main> }
            }
            FetchState::Success(e) => html! {
                <main>
                    <div class="presentation">
                        <div>
                            <h1>{&e.title}</h1>
                            <p class="info">{"publicat de "}<Link<Route> classes={classes!("subtitle", "is-block")} to={Route::Author { id: e.author.id }}>{ &e.author.name }</Link<Route>>{format!(" în {}/{}/{}", &e.created_on.day0(), &e.created_on.month0(), &e.created_on.year())}</p>
                            <p>{&e.description}</p>
                        </div>
                        <aside><img alt="This post's image" src={e.image_url.clone()} loading="lazy" /></aside>
                    </div>
                </main>
            },
            FetchState::Failed(e) => html! {
                <main>
                    <h2 class="error">{"Fetch Failed!"}</h2>
                    <p>{format!("Error: {}", e)}</p>
                </main>
            }
        }
    }
}

#[function_component(About)]
fn about() -> Html {
    html! {
        <main>
            <h1>{"Adela Margin"}</h1>
            <p>{"Coach leadership si relatii interumane"}</p>
            <p>{r#"In formare si indragostita de  "Terapia sistemica centrata pe emotii" pe care o folosesc mult in lucru cu cuplurile (https://www.psiris.ro/)"#}</p>
            <p>{r#""Emotional Focused Therapy" - are o rata mare de succes in randul cuplurilor, cu o eficienta dovedita de 70% (https://www.eftromania.ro/)"#}</p>
            <p>{r#"Educator parental "Cercul Sigurantei""#}</p>
            <p>{r#"Consultant leadership certificat in Agile cu experienta in implementarea SAFe si SCRUM in diverse companii"#}</p>
            <p>{r#"Membru al Asociatiei "Inceputuri Sigure Romania" - ONG cu rolul de a ajuta la formarea de atasamente sigure in medii defavorizate"#}</p>
            <p>{r#"Ce ma recomanda: Empatia si ascultarea activa"#}</p>
            <p>{r#"Faptul ca am trecut eu insami prin workaholism, burnout, depresie, anxietate si am ajuns sa ma bucur din nou de viata, imi da incredere ca pot sa ii ajut si pe altii sa vada lumina si caldura din suflete."#}</p>
            <p>{r#"Iar metodele si tehnicile folosite de mine lucreaza foarte eficient in alinarea durerii si retrezirea pas cu pas a poftei de viata si a iubirii din noi."#}</p>
            <p>{r#"In cateva cuvinte: Iubirea fata de oameni si de viata!"#}</p>
            <p>{r#"Alte formari/cursuri"#}</p>
            <p>{r#"Sexualitate si explorare in siguranta, Coaching, Curs Terapia Somatica a Traumei Peter Levine&co - embodylab.com, Train of Trainers, Externship Emotionational Focused Therapy - Simona Herb, Practica in Terapia prin acceptare si angajament, NLP Practitioner, Stiinta cuplului - Domnica Petrovai. Stress Management si Motivare - Mircea Miclea"#}</p>
            <h2>{"Cum am ajuns cine sunt"}</h2>
            <p>{"Sunt Adela - mama, psihoeducator, leader, suflet si ajutor de suflete. Si FEMEIE, sa nu uitam."}</p>
            <p>{"Jonglez zilnic intre toate aceste roluri, uneori ca intr-un dans, alteori ca intr-o furtuna si cred cu tarie ca IMPREUNA, putem fi mai mult decat suma partilor!"}</p>
            <p>{"Cam de cand ma stiu sunt fascinata de relatiile interumane, si in mod special de cele de cuplu si manifestarea senzualitatii in viata noastra zilnica, ca un indicator al starii de bine si al poftei de viata. Iar cand spun senzualitate, spun multe lucruri."}</p>
            <p>{"Imi plac muntii, sporturile si socializarea, dar si mai mult de atat…conectarea interumana: adica acel sentiment cand simti iubirea in tine si sa daruiesti devine o placere. Cand simti ca totul se opreste in jur si esti placut, vazut si apreciat exact asa cum esti …si culmea, ca in acele momente, tu te placi pe tine cel mai mult, iar asta doar se reflecta in ceilalti! Ai observat asta?"}</p>
            <p>{"Eu conectarea asta o obtin cel mai usor, in viata mea, prin umor, ironie si vulnerabilitate. Astfel ca-mi place sa ies in lume si sa creez relatii pe baza de aceste principii care-mi permit sa vad oamenii pe dinauntru. Si mereu e   un mare DAR cand ajung sa vad pe cineva pe dinauntru."}</p>
            <p>{"Si ajut cu mare drag cuplurile sa-si poata face aceste daruri emotionale sesiune de sesiune. Sa invete cum sa-si raspunda astfel incat sa poata primi aceste daruri."}</p>
            <p>{"Caci, conform Teoriei atasamentului, sistemul nervos se linisteste cel mai bine si mai usor in prezenta unei persoane dragi, cu care suntem in legatura armonioasa, adica conectati."}</p>
            <p>{"Iar ca persoana care nu esti parte dintr-o relatie, aceasta stare poate fi invatata, la inceput, in  relatia terapeutica, ca apoi, sa poti sa-ti iei zborul in lume si sa-ti gasesti acel ceva care e pentru tine."}</p>
            <p>{"Pe principii similare se deruleaza si relatiile noastre de la job, asa ca te pot sprijini ca lider, in drumul tau spre a-ti face munca cu mai putin stres si cu rezultate foarte bune, pastrand armonia in jurul tau. Iar toti vor vrea apoi sa faca parte din echipa ta, ca doar stii…oamenii nu parasesc firmele, ci sefii, nu-i asa?"}</p>
            <p>{"Despre toate astea si multe altele, intr-un proces terapeutic, in care te poti vindeca intr-o relatie   de siguranta, empatie si acceptare. Detalii despre programari: "}<a href="tel:+40740056316">{"+40740056316"}</a></p>
            <p>{"Daca vrei doar sa ne cunoastem, sa vezi cum rezonam, poti sa te inscrii la o sedinta gratuita, mai jos."}</p>
            <p>{"Fac cu mult drag si grupuri de Cercul sigurantei pentru cei care-si doresc sa invete despre parenting intr-un mod realist si intr-un program bazat pe studii ce stau la baza dezvoltarii umane. Mediul e unul cald, de acceptare si siguranta in care invatam sa modelam relatiile cu copiii nostri si cu cei din jur. Si inainte de toate, Invatam CONECTAREA sau A FI ALATURI, astfel incat relatiile cu cei din jur sa devina implinitoare. Plus, odata ce ai platit pentru Cerc, poti participa ori de cate ori mai doresti sa-ti improspatezi informatia din curs, din nou, cu alta grupa."}</p>
            <p>{"Pentru inscrieri in grupele urmatoare, te poti inscrie aici sau printr-un mesaj pe whatsapp:"}</p>
            <p><a href={"https://tinyurl.com/yc4ce8jj"}>{"https://tinyurl.com/yc4ce8jj"}</a></p>
            <p>{"Cu grija de tine!"}</p>
        </main>
    }
}

#[function_component(Schedule)]
fn schedule() -> Html {
    html! {
        <main>
            <div class="calendly-inline-widget" data-url="https://calendly.com/adel-druhora/60min" style="min-width:320px;height:630px;"></div>
            <script type="text/javascript" src="https://assets.calendly.com/assets/external/widget.js" async=true></script>
        </main>
    }
}

#[function_component(Contact)]
fn contact() -> Html {
    html! {
        <main>
            <h1>{"Art of Feeling"}</h1>
            <h2>{"Adela Margin"}</h2>
            <p>{"Cluj-Napoca, str. Luncii, nr. 49"}</p>
            <p><a href="tel:+40740056316">{"+40740056316"}</a></p>
            <h2>{"Lăsați-ne un mesaj și vă contactăm noi"}</h2>
            <form>
            <p>{"Completați cel puțin un câmp dintre e-mail și telefon ca să vă putem contacta, dacă sunteți ok cu ambele metode le puteți completa ambele!"}</p>
            <p><label>{"Numele dumneavoastră "}<span>{"(optional)"}</span><br/><input type="text"/></label></p>
            <p><label>{"Adresă e-mail "}<span>{"(optional)"}</span><br/><input type="email"/></label></p>
            <p><label>{"Număr telefon "}<span>{"(optional)"}</span><br/><input type="tel"/></label></p>
            <p><label>{"Motivul pentru care doriți să luăm legătura"}<br/><textarea/></label></p>
            <p><input type="submit" value="Trimite mesajul"/></p>
            </form>
        </main>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Schedule));
    html! {
        <main>
            <div class="presentation">
            <div>
            <h1>{"Arta de a simți"}</h1>
            <p>{"Te ajutăm să deprinzi arta de a trăi armonios alături de ceilalți; atât în viața personală, cât și în cea profesională și anume arta de a simți cu adevărat viața!"}</p>
            </div>
            <aside><img src="/static/aof1.jpg"/><button {onclick}>{"Programează o ședință"}</button></aside>
            </div>
            <h2>{"Cine suntem"}</h2>
            <p>{"ArtofFeeling e o comunitate de psihoeducatori"}</p>
            <p>{"Suntem o mână de psihoeducatori și ne dorim să vă ajutăm să ajungeți la cea mai bună versiune a voastră pentru a vă bucura din plin de relațiile din jurul vostru, căci ele sunt cele care ne aduc la final de zi și de viață, starea de bine, armonia."}</p>
            <h2>{"Ce facem"}</h2>
            <p>{"ArtofFeeling însoțește pe cei în nevoie în călătoria lor de cunoaștere. Astfel, ei reînvață să relaționeze într-un mod sănătos cu cei din jur pentru a avea o viață cu vise împlinite și pentru a-și găsi scopul în comunitate fără a-și sacrifica starea de bine și autonomia."}</p>
            <p>{"Cu mare grijă și empatie îi ajutăm pe cei în suferință să dea un sens durerii folosind metode inedite din Terapia Somatică, Pozitivă și Centrată pe Emoții. Și pe alocuri, niste ACT."}</p>
            <h2>{"Oferta noastră"}</h2>
            <p>{"Art of Feeling vă oferă servicii de"}</p>
            <ul>
                <li>{"Terapie online și offline în Cluj-Napoca"}</li>
                <li>{"Terapie de cuplu, individuală și de familie"}</li>
                <li>{"Cercul Siguranței pentru familii și cupluri"}</li>
                <li>{"Workshop-uri de educație parentală și grijă de sine"}</li>
                <li>{"Workshop-uri de dezvoltare a abilităților de relaționare și leadership"}</li>
                <li>{"Comunități de sprijin pentru cupluri și familii"}</li>
            </ul>
            <h2>{"Trainiguri și psihoeducație"}</h2>
            <ul>
                <li>{"Coaching și traininguri specifice pentru companii"}</li>
                <li>{"Agile și Leadership Coaching"}</li>
                <li>{"Consultanță Agile"}</li>
                <li>{"Programe personalizate de implementare a principiilor Agile în companii"}<ul>
                    <li>{"Implementare și training SCRUM și SAFe"}</li>
                    <li>{"Training și coaching pentru leaderi"}</li>
                </ul></li>
            </ul>
        </main>
    }
}

struct App {
    //     _listener: HistoryListener,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            //             _listener: listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetBlogFetchState(_) => todo!(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <BrowserRouter>
                <header>
                <nav>
                <Link<Route> to={Route::Home}>{ "Art of Feeling" }</Link<Route>>
                <input type="checkbox" id="showmenu"/><label for="showmenu">{"≡"}</label>
                <Link<Route> to={Route::About}>{ "Despre noi" }</Link<Route>>
                <Link<Route> to={Route::Services}>{ "Servicii" }</Link<Route>>
                <Link<Route> to={Route::Blog { id: 1 }}>{ "Blog" }</Link<Route>>
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
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Author { id } => {
            html! {<main><p>{format!("You are looking at Author {}", id)}</p></main>}
        }
        Route::Services => html! { <About /> },
        Route::Blog { id } => html! { <Blog id={*id}/> },
        Route::Schedule => html! { <Schedule /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::start_app::<App>();
}
