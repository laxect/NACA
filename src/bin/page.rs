use naca::{NACAAirfoil, NACA4};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlTextAreaElement, WebGlRenderingContext};
use yew::{
    events::Event, function_component, html, use_node_ref, use_state_eq, Callback, TargetCast,
};

fn render(gl: WebGlRenderingContext, number: u32) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = gl.create_buffer().unwrap();
    let naca4: NACA4 = format!("{}", number).parse()?;

    let point_number = 10000;
    let u: Vec<f32> = (0..=point_number)
        .map(|x| x as f32 / point_number as f32)
        .flat_map(|x| [naca4.xu(x), naca4.yu(x)])
        .collect();
    let l: Vec<f32> = (0..=point_number)
        .map(|x| x as f32 / point_number as f32)
        .flat_map(|x| [naca4.xl(x), naca4.yl(x)])
        .rev()
        .collect();

    let m: Vec<f32> = (0..=point_number)
        .map(|x| x as f32 / point_number as f32)
        .flat_map(|x| [x, naca4.yc(x)])
        .collect();

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    let vector = [u, l, m].concat();
    let vector = js_sys::Float32Array::from(vector.as_slice());
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vector,
        WebGlRenderingContext::STATIC_DRAW,
    );
    gl.draw_arrays(
        WebGlRenderingContext::LINE_STRIP,
        0,
        point_number as i32 * 3,
    );
    Ok(())
}

#[function_component(Zone)]
fn zone() -> Html {
    let number = use_state_eq(|| 2412u32);
    let onchange = {
        let data = number.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlTextAreaElement>();
            if let Some(input) = target.map(|input| input.value()) {
                if let Ok(input) = input.parse() {
                    data.set(input);
                }
            }
        })
    };

    let canvas_ref = use_node_ref();
    if let Some(node) = canvas_ref.cast::<HtmlCanvasElement>() {
        let gl: WebGlRenderingContext = node
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        if let Err(e) = render(gl, *number) {
            gloo::console::error!(e.to_string());
        };
    }
    html! {
        <div style="width:80%;margin-left:auto;margin-right:auto;">
        <textarea id="input" rows=2 name="data" style="width:100vh;" {onchange} placeholder={"2412"} />
        <canvas ref={canvas_ref} />
        </div>
    }
}

fn main() {
    yew::start_app::<Zone>();
}
