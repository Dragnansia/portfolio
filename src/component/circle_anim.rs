use yew::prelude::*;

pub struct CircleAnim {
    count: i32,
}

impl Component for CircleAnim {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { count: 10 }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let circles = vec![html! { <li> </li> }; self.count as usize];

        html! {
            <div>
                <ul class="circles">
                    { for circles }
                </ul>
            </div >
        }
    }
}
