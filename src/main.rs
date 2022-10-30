use std::thread::current;

use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast, JsValue};

fn main(){
    let nums: Vec<f64> = vec![0.55,1.00,2.20,4.00,6.50,12.00,16.00];
    let mut nums_new: Vec<f64> = Vec::new();
    for n in nums{
        nums_new.push(n.powi(3));
    }
    println!("{:?}",nums_new);
    //yew::start_app::<MainComponent>();
}

//vec must be sorted
fn max_duplicates(vals: &Vec<f64>) -> u32{
    let mut max: u32 = 0;
    let mut current_dupes: u32 = 0;
    let mut prev: &f64 = vals.first().unwrap_or(&0.0);
    for v in vals{
        if v==prev{
            current_dupes+=1;
        }else{
            if current_dupes>max{
                max=current_dupes;
            }
            current_dupes=1;
        }
        prev=v;
    }
    if current_dupes>max{
        max=current_dupes;
    }
    current_dupes=1;
    return max
}

fn tick_scale(range: f64) -> f64{
    let scale_power: f64 = range.log10();
    let decimals: f64 = scale_power%1.0;
    let scales: Vec<f64> = vec![1.0,2.0,5.0,10.0];
    let mut minscale: f64 = 1.0;
    for v in scales{
        if ((decimals-minscale.log10()).abs())>((decimals-v.log10()).abs()){
            minscale=v;
        }
    }
    let pow = if (minscale-1.0).abs()<0.1 && decimals>0.5{
        scale_power as i32 
    }else{
        scale_power as i32 - 1
    };
    return 10_f64.powi(pow)*minscale;
}

struct MainComponent{
    temp: i32
}
impl Component for MainComponent{
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{temp: 5}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //let test_vals: Vec<f64> = vec![1.0,2.0,2.0,2.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,4.0,4.0];
        let test_vals: Vec<f64> = vec![1.0,1.0,2.0,2.0,2.0,3.0,3.0,3.0,3.0,4.0,4.0,4.0,5.0,5.0,6.0];
        //background, secondary, text, accent, lines
        //#505050 or #808080 #ffffff for text
        let colors = vec!["#303030".to_string(), "#404040".to_string(), "#808080".to_string(), "#e2b831".to_string(), "#000000".to_string()];
        html!{
            <>
                <InputGrid />
                <DotPlotComponent points={test_vals.clone()} width={1024.min((0.9*web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap_or(200.0)) as u32)} height=200 radius=5.0 colors={colors.clone()} />
                <Bargraph width={1024.min((0.9*web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap_or(200.0)) as u32)} height={512} x_range={(0.0,100.0)} y_range={(0.0,100.0)} colors={colors.clone()} labels={(String::from(""), String::from(""))}>
                    <BargraphBar color={colors[3].clone()} width={(10.0,20.0)} height={(0.0,90.0)} label={String::from("")}/>
                    <BargraphBar color={colors[3].clone()} width={(20.0,30.0)} height={(0.0,50.0)} label={String::from("")}/>
                    <BargraphBar color={colors[3].clone()} width={(30.0,40.0)} height={(50.0,100.0)} label={String::from("")}/>
                </Bargraph>
                <BoxplotComponent width={1024.min((0.9*web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap_or(200.0)) as u32)} height={256} point_summery={PointSummery::create(test_vals.clone())} colors={colors.clone()} label={String::from("")}/>
            </>
        }
    }
}

enum PlotMsg{
    Hover(i32),
    Update,
    None
}

#[derive(Clone, PartialEq, Properties)]
struct DotplotProps{
    points: Vec<f64>,
    width: u32,
    height: u32,
    radius: f64,
    colors: Vec<String>
}

struct DotPlotComponent{
    canvas: NodeRef,
    most_num_vals: u32,
    range: (f64, f64)
}

impl Component for DotPlotComponent{
    type Message = PlotMsg;
    type Properties = DotplotProps;
    fn create(_ctx: &Context<Self>) -> Self{
        //let link = _ctx.link();
        //why do i need _v
        //link.callback(|_v: u8| PlotMsg::Update);

        Self{canvas: NodeRef::default(), most_num_vals: max_duplicates(&_ctx.props().points), range: (*_ctx.props().points.first().unwrap_or(&0.0),*_ctx.props().points.last().unwrap_or(&0.0))}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg{
            PlotMsg::Update => {
                true
            }
            PlotMsg::Hover(position) => {
                false
            }
            PlotMsg::None => {
                //PreviousInput::create(yew::);
                false
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html{
        let link = _ctx.link();
        let radius = _ctx.props().radius;
        html! {
            <div class = "center-block">
                <canvas class="center-block" width={_ctx.props().width.to_string()} height={((((radius*2.2) as u32)*self.most_num_vals+30).max(_ctx.props().height)).to_string()} ref={self.canvas.clone()}/>
            </div>
        }
    }
    // WARNING!!!! SORT THE PROPS BEFORE RUNNING THIS OR ELSE EVERYTHING BREAKSSS!!!!!
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let width: f64 = ctx.props().width as f64;
        let buffer: f64 = 10.0;
        let radius: f64 = ctx.props().radius;
        let buffered_width: f64 = width-buffer*2.0;
        let height: f64 = ((((radius*2.2) as u32)*self.most_num_vals+30).max(ctx.props().height)) as f64;
        let canvas_ref: HtmlCanvasElement = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        let context: CanvasRenderingContext2d = canvas_ref.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
        let colors = &ctx.props().colors;
        //sets up scale
        let scale: f64 = tick_scale(self.range.1-self.range.0);
        let start: f64 = if self.range.0%scale==0.0 {self.range.0.clone()} else {self.range.0-self.range.0%scale+scale};
        //draw line
        draw_numberline(&context, self.range, (buffer,height-20.0), (buffer+buffered_width, height-20.0), "15px Verdana", -10.0, &colors[4], &colors[2]);
        /*
        context.begin_path();
        context.set_fill_style(&JsValue::from_str(&colors[2]));
        context.move_to(0.0, height-20.0);
        context.line_to(width,height-20.0);
        context.set_text_align("center");
        context.set_font("15px Verdana");
        let mut num = start;
        let roundto: usize = if scale.log10()<0.0 {scale.log10() as usize + 1_usize} else {0_usize};
        while num<=self.range.1{
            context.fill_text(&num.to_string(), buffer+(num-self.range.0)/(self.range.1-self.range.0)*buffered_width, height).unwrap();
            num+=scale;
            num=format!("{:.1$}",num,roundto).parse::<f64>().unwrap_or(0.0);
        }
        context.stroke();*/
        //draw dots
        context.begin_path();
        let mut prev: f64 = self.range.0;
        let mut iter: f64 = 0.0;
        for p in &ctx.props().points{
            if &prev!=p{
                iter=0.0;
            }
            let xcord = buffer+(p-self.range.0)/(self.range.1-self.range.0)*buffered_width;
            let ycord = height-30.0-radius*2.2*iter;
            context.move_to(xcord, ycord);
            context.arc(xcord,ycord,radius,0.0,std::f64::consts::PI*2.0).unwrap();
            iter+=1.0;
            prev=p.clone();
        }
        context.set_stroke_style(&JsValue::from_str(&colors[3]));
        context.set_fill_style(&JsValue::from_str(&colors[3]));
        context.fill();
        context.stroke();
    }
}

#[derive(Clone, PartialEq, Default)]
struct PointSummery{
    mean: f64,
    median: f64,
    min: f64,
    max: f64, 
    q1: f64,
    q3: f64,
    standard_deviation: f64,
    values: Vec<f64>,
}
impl PointSummery{
    fn create(mut values: Vec<f64>) -> Self{
        if values.is_empty(){
            return Self::default()
        }
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let min: f64 = values.first().unwrap_or(&0.0).clone();
        let max: f64 = values.last().unwrap_or(&0.0).clone();
        let mean: f64 = if values.len()==0 {0.0} else {values.iter().sum::<f64>()/values.len() as f64};
        let medloc: f64 = ((values.len() - 1) as f64) / 2.0;
        let median: f64 = if values.len()<3 {mean.clone()} else if medloc%1.0>0.01 {(values[medloc as usize]+values[medloc as usize + 1_usize]) as f64 / 2.0} else {values[medloc as usize]};
        let q1: f64 = if values.len()<3 {min.clone()} else if (medloc/2.0)%1.0>0.01 {(values[(medloc-medloc/2.0) as usize]+values[(medloc-medloc/2.0) as usize + 1_usize]) as f64 / 2.0} else {values[(medloc-medloc/2.0) as usize]};
        let q3: f64 = if values.len()<3 {max.clone()} else if (medloc/2.0)%1.0>0.01 {(values[(medloc+medloc/2.0) as usize]+values[(medloc+medloc/2.0) as usize + 1_usize]) as f64 / 2.0} else {values[(medloc+medloc/2.0) as usize]};
        let varience: f64 = values.iter().map(|v| {
            let diff: f64 = v - mean;
            diff*diff
        }).sum::<f64>() / values.len() as f64;
        let sd: f64 = varience.sqrt();
        Self{min, max, mean, median, q1, q3, standard_deviation: sd, values}
    }
    fn range_excluding_outliers(&self) -> (f64, f64){
        let iqr: f64 = self.q3-self.q1;
        let upper_fence: f64 = self.q3+1.5*iqr;
        let lower_fence: f64 = self.q1-1.5*iqr;
        (
            if self.min>lower_fence {self.min.clone()} else {self.values.iter().find(|&v| v>=&lower_fence).unwrap_or(&self.min).clone()},
            if self.max<upper_fence {self.max.clone()} else {self.values.iter().rev().find(|&v| v<=&upper_fence).unwrap_or(&self.max).clone()}
        )
    }
    fn outliers(&self) -> Vec<f64>{
        let iqr: f64 = self.q3-self.q1;
        let upper_fence: f64 = self.q3+1.5*iqr;
        let lower_fence: f64 = self.q1-1.5*iqr;
        if self.min>lower_fence && self.max<upper_fence{
            return Vec::new()
        }
        let mut res: Vec<f64> = Vec::new();
        if self.min<lower_fence{
            res.extend(self.values.iter().take_while(|v| **v<lower_fence));
        }
        if self.max>upper_fence{
            res.extend(self.values.iter().rev().take_while(|v| **v>upper_fence));
        }
        res
    }
}
struct HistogramProps{
    width: u32,
    height: u32,
    dimentions: (f64, f64),
    labels: (String, String)
}

#[derive(Clone, PartialEq, Properties)]
struct BoxplotProps{
    width: u32,
    height: u32,
    colors: Vec<String>,
    label: String,
    point_summery: PointSummery
}
struct BoxplotComponent{
    canvas: NodeRef,
}
impl Component for BoxplotComponent{
    type Message = PlotMsg;
    type Properties = BoxplotProps;
    fn create(_ctx: &Context<Self>) -> Self{
        Self{canvas: NodeRef::default()}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class = "center-block">
                <canvas class="center-block" width={ctx.props().width.to_string()} height={ctx.props().height.to_string()} ref={self.canvas.clone()}/>
            </div>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let canvas_ref: HtmlCanvasElement = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        let context: CanvasRenderingContext2d = canvas_ref.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
        let casted_dim: (f64, f64) = (ctx.props().width as f64, ctx.props().height as f64);
        let pts: &PointSummery = &ctx.props().point_summery;
        let graph_x_range: (f64, f64) = (10.0,casted_dim.0-10.0);
        let graph_y_range: (f64, f64) = (casted_dim.1-20.0, 0.0);
        let bar_buffer: f64 = 20.0;
        let num_bars: f64 = 1.0;
        let outlier_radius: f64 = 5.0;
        let single_bar_height: f64 = (graph_y_range.0)/num_bars-bar_buffer; // set div to # bars
        //line
        draw_numberline(&context, (pts.min,pts.max), (graph_x_range.0,graph_y_range.0), (graph_x_range.1,graph_y_range.0), "15px Verdana", -10.0, &ctx.props().colors[4], &ctx.props().colors[2]);
        //lines for boxes
        let non_outlier_range: (f64, f64) = pts.range_excluding_outliers();
        let temp: f64 = 0.0;
        let current_bar_y_range: (f64, f64) = (graph_y_range.0-temp*((graph_y_range.0-graph_y_range.1).abs()/num_bars)-bar_buffer,graph_y_range.0-(temp+1.0)*((graph_y_range.0-graph_y_range.1).abs()/num_bars));
        context.begin_path();
        context.set_fill_style(&JsValue::from_str(&ctx.props().colors[3]));
        context.move_to(map_value(non_outlier_range.0, (pts.min,pts.max), graph_x_range),current_bar_y_range.0-single_bar_height/2.0);
        context.line_to(map_value(non_outlier_range.1, (pts.min,pts.max), graph_x_range),current_bar_y_range.0-single_bar_height/2.0);
        context.move_to(map_value(non_outlier_range.0, (pts.min,pts.max), graph_x_range), current_bar_y_range.0-single_bar_height/4.0);
        context.line_to(map_value(non_outlier_range.0, (pts.min,pts.max), graph_x_range),current_bar_y_range.1+single_bar_height/4.0);
        context.move_to(map_value(non_outlier_range.1, (pts.min,pts.max), graph_x_range), current_bar_y_range.0-single_bar_height/4.0);
        context.line_to(map_value(non_outlier_range.1, (pts.min,pts.max), graph_x_range),current_bar_y_range.1+single_bar_height/4.0);
        context.stroke();
        //boxes and circles
        context.begin_path();
        context.set_stroke_style(&JsValue::from_str(&ctx.props().colors[3]));
        context.set_fill_style(&JsValue::from_str(&ctx.props().colors[3]));
        for v in pts.outliers(){
            let cord: (f64, f64) = (map_value(v,(pts.min,pts.max),graph_x_range),current_bar_y_range.0-single_bar_height/2.0);
            context.move_to(cord.0, cord.1);
            context.arc(cord.0,cord.1,outlier_radius,0.0,std::f64::consts::PI*2.0).unwrap();
        }
        context.fill();
        fill_rect_xy(&context, map_value(pts.q1, (pts.min,pts.max), graph_x_range), current_bar_y_range.0, 
        map_value(pts.q3, (pts.min,pts.max), graph_x_range), current_bar_y_range.1);
        context.stroke();
        context.begin_path();
        context.move_to(map_value(pts.median, (pts.min,pts.max), graph_x_range), current_bar_y_range.0);
        context.set_stroke_style(&JsValue::from_str(&ctx.props().colors[4]));
        context.line_to(map_value(pts.median, (pts.min,pts.max), graph_x_range),current_bar_y_range.1);
        context.stroke();
    }
}

fn fill_rect_xy(context: &CanvasRenderingContext2d, x1: f64, y1: f64, x2: f64, y2: f64){
    context.fill_rect(x1, y1, x2-x1, y2-y1)
}

#[derive(Clone, PartialEq, Properties)]
struct BargraphBarProps{
    color: String,
    width: (f64, f64),
    height: (f64, f64),
    label: String,
}

struct BargraphBar;
impl Component for BargraphBar{
    type Message = ();
    type Properties = BargraphBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{

        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct BargraphProps{
    children: ChildrenWithProps<BargraphBar>,
    width: u32,
    height: u32,
    y_range: (f64, f64),
    x_range: (f64, f64),
    colors: Vec<String>,
    labels: (String, String)
}
struct Bargraph{
    canvas: NodeRef
}
impl Component for Bargraph{
    type Message = ();
    type Properties = BargraphProps;
    fn create(ctx: &Context<Self>) -> Self {
        Self{canvas: NodeRef::default()}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{ 
            <div class = "center-block">
                <canvas class="center-block" width={ctx.props().width.to_string()} height={ctx.props().height.to_string()} ref={self.canvas.clone()}/>
            </div>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool){
        let canvas_ref: HtmlCanvasElement = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        let context: CanvasRenderingContext2d = canvas_ref.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
        let casted_px_dim: (f64, f64) = (ctx.props().width as f64, ctx.props().height as f64);
        let graph_x_range: (f64, f64) = (30.0,casted_px_dim.0-20.0);
        let graph_y_range: (f64, f64) = (casted_px_dim.1-20.0,10.0);
        //draws axis
        context.begin_path();
        context.set_fill_style(&JsValue::from_str(&ctx.props().colors[3]));
        for child in ctx.props().children.clone(){
            let props = &child.props;
            fill_rect_xy(&context, map_value(props.width.0,ctx.props().x_range,graph_x_range),
            map_value(props.height.0,ctx.props().y_range,graph_y_range),
            map_value(props.width.1,ctx.props().x_range,graph_x_range),
            map_value(props.height.1,ctx.props().y_range,graph_y_range));
            //context.fill_rect(map_value(props.width.0,ctx.props().x_range,graph_x_range),
            //map_value(props.height.0,ctx.props().y_range,graph_y_range),
            //map_value(props.width.1-props.width.0,ctx.props().x_range,(0.0,graph_x_range.1-graph_x_range.0)),
            //map_value(-props.height.1+props.height.0,ctx.props().y_range,(0.0,graph_y_range.0-graph_y_range.1)))
        }
        context.stroke();
        draw_numberline(&context, ctx.props().x_range, (graph_x_range.0,graph_y_range.0), (graph_x_range.1,graph_y_range.0), "15px Verdana", -10.0, &ctx.props().colors[4], &ctx.props().colors[2]);
        draw_numberline(&context, ctx.props().y_range, (graph_x_range.0,graph_y_range.0), (graph_x_range.0,graph_y_range.1), "15px Verdana", 3.0, &ctx.props().colors[4], &ctx.props().colors[2]);
        
    }
}

fn map_value(value: f64, from_scale: (f64, f64), to_scale: (f64, f64)) -> f64{
    return (value-from_scale.0)/(from_scale.1-from_scale.0)*(to_scale.1-to_scale.0)+to_scale.0
}

fn draw_numberline(context: &CanvasRenderingContext2d, range: (f64, f64), start: (f64, f64), end: (f64, f64), textfont: &str, textdist: f64, linecolor: &str, textcolor: &str){
    let scale: f64 = tick_scale(range.1-range.0);
    let start_val: f64 = if range.0%scale==0.0 {range.0.clone()} else {range.0-range.0%scale+scale};
    let font_size: f64 = textfont.split_once("px").unwrap_or(("12","Verdana")).0.parse::<f64>().unwrap_or(12.0);
    let cord_range: (f64, f64)=(end.0-start.0, end.1-start.1);
    let line_direction: (f64, f64) = normalize_2dvec(cord_range);
    let line_normal: (f64, f64) = (line_direction.1, -line_direction.0);
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str(linecolor));
    context.set_fill_style(&JsValue::from_str(textcolor));
    context.move_to(start.0, start.1);
    context.line_to(end.0, end.1);
    if range.1==range.0{
        context.stroke();
        return
    }
    if line_normal.0*textdist.signum() > 0.8{context.set_text_align("left")}
    else if line_normal.0*textdist.signum() < -0.8{context.set_text_align("right")}
    else{context.set_text_align("center")};
    context.set_font(textfont);
    let text_start: (f64, f64) = (start.0+line_normal.0*textdist,start.1+line_normal.1*textdist);
    let roundto: usize = if scale.log10()<0.0 {scale.log10() as usize + 1_usize} else {0_usize};
    let mut num: f64 = start_val;
    while num<=range.1{
        let num_percent: f64 = (num-range.0)/(range.1-range.0);
        context.fill_text(&num.to_string(), text_start.0+num_percent*cord_range.0, text_start.1+num_percent*cord_range.1+font_size/2.0).unwrap();
        num+=scale;
        num=format!("{:.1$}",num,roundto).parse::<f64>().unwrap_or(0.0);
    }
    context.stroke();
}

fn normalize_2dvec(vector: (f64, f64)) -> (f64, f64){
    let mag: f64 = (vector.0*vector.0+vector.1*vector.1).sqrt();
    return (vector.0/mag, vector.1/mag);
}

enum InputMsg{
    Input(String, u32),
    Enter,
    None
}

struct InputGrid{
    content: Vec<String>,
    width: u32,
    height: u32,
    xlabels: Vec<String>,
    ylabels: Vec<String>
}

impl Component for InputGrid{
    type Message = InputMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self{
        Self{content: Vec::new(), width: 2, height: 2, xlabels: Vec::new(), ylabels: Vec::new()}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg{
            InputMsg::Input(content, idx) => {
                false
            }
            InputMsg::Enter => {
                true
            }
            InputMsg::None => {
                //PreviousInput::create(yew::);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class="center-block">
                <input class="center-block bgcol1" />
            </div>
        }
    }
}

