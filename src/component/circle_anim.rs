use yew::{prelude::*, virtual_dom::VNode};

pub struct CircleAnim {
    count: i32,
}

impl Component for CircleAnim {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { count: 30 }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let circles = (0..self.count).map(|_| random_li()).collect::<Vec<VNode>>();

        html! {
            <div>
                <ul class="circles" id="circles">
                    { for circles }
                </ul>
            </div >
        }
    }
}

fn random_li() -> Html {
    let size = random_number(30, 100);
    let left = random_number(10, 90);
    let anim_delay = random_number(0, 10);
    let anim_duration = random_number(5, 45);

    let style = format!(
        "width:{size}px;height:{size}px;left:{}%;animation-delay:{}s;animation-duration:{}s",
        left,
        anim_delay,
        anim_duration,
        size = size
    );

    html! { <li style={ style.clone() } /> }
}

fn random_number(min: i32, max: i32) -> i32 {
    let mut val = [0u8; 1];
    getrandom::getrandom(&mut val).unwrap();
    val[0] as i32 % max + min
}
