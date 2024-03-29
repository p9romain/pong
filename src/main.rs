use sdl2::{
  pixels::Color, 
  event::Event, 
  keyboard::Keycode, 
  rect::Rect, 
  render::Canvas, 
  video::Window, 
} ;
use std::{
  time::Duration,
  path::Path,
  f64::consts::PI,
} ;

fn draw_rec(canvas : &mut Canvas<Window>, c : Color, x : i32, y : i32, width : i32, height : i32)
{
  canvas.set_draw_color(c) ;
  canvas.fill_rect(Rect::new(x, y, width as u32, height as u32)).expect("couldn't fill rect") ;
}

fn main() 
{
  let (w_width, w_height) = (1080i32, 720i32) ;

  let sdl_context = sdl2::init().expect("couldn't initialize SDL2") ;
  let video_subsystem = sdl_context.video().expect("couldn't initialize video subsystem") ;

  let window = video_subsystem.window("pong", w_width as u32, w_height as u32)
    .position_centered()
    .build()
    .expect("couldn't create a window") ;

  let mut canvas = window.into_canvas().build().expect("couldn't create a canva") ;
  let texture_creator = canvas.texture_creator() ;

  // Prepare font
  let ttf_context = sdl2::ttf::init().unwrap() ;
  let font_path = Path::new(&"fonts/Minecraftia-Regular.ttf") ;
  let mut font = ttf_context.load_font(font_path, 128).expect("couldn't load the font") ;
  font.set_style(sdl2::ttf::FontStyle::BOLD) ;

  // Other.. thing
  // Stuff
  let mut event_pump = sdl_context.event_pump().expect("couldn't get event pump(?)") ;

  // Var
  let (mut hold_up_p1, mut hold_down_p1, mut hold_up_p2, mut hold_down_p2) = (false, false, false, false) ;

  let paddle_pos_compared_to_w_side = w_width/10 ;

  let ball_size : i32 = 20 ;
  let (mut ball_x, mut ball_y) = (w_width/2, w_height/2) ;
  let ball_speed : i32 = 12 ;
  let mut angle : f64 = 0.0 ;

  let (paddle_size_w, paddle_size_h) = (15, 100) ;
  let (paddle_p1_x, mut paddle_p1_y) = (paddle_pos_compared_to_w_side, w_height/2) ;
  let (paddle_p2_x, mut paddle_p2_y) = (w_width - paddle_pos_compared_to_w_side, w_height/2) ;
  let paddle_speed = 8 ;

  let paddle_y_limit_min = paddle_size_h/2 ;
  let paddle_y_limit_max = w_height - paddle_size_h/2 ;

  let mut score_p1 : u32 = 0 ;
  let mut score_p2 : u32 = 0 ;

  let mut game = false ;
  let mut first_time = true ;

  // Loop
  'run : loop
  {
    canvas.set_draw_color(Color::RGB(50, 50, 50)) ;
    canvas.clear() ;

    // key listener
    for evt in event_pump.poll_iter()
    {
      match evt 
      {
        Event::Quit {..} | 
        Event::KeyDown { keycode : Some(Keycode::Escape), .. } =>
        {
          break 'run 
        },
        Event::KeyDown { keycode : Some(Keycode::Z), .. } =>
        {
          hold_up_p1 = true ;
        },
        Event::KeyDown { keycode : Some(Keycode::S), .. } =>
        {
          hold_down_p1 = true ;
        },
        Event::KeyDown { keycode : Some(Keycode::Up), .. } =>
        {
          hold_up_p2 = true ;
        },
        Event::KeyDown { keycode : Some(Keycode::Down), .. } =>
        {
          hold_down_p2 = true ;
        },
        Event::KeyUp { keycode : Some(Keycode::Z), .. } =>
        {
          hold_up_p1 = false ;
        },
        Event::KeyUp { keycode : Some(Keycode::S), .. } =>
        {
          hold_down_p1 = false ;
        },
        Event::KeyUp { keycode : Some(Keycode::Up), .. } =>
        {
          hold_up_p2 = false ;
        },
        Event::KeyUp { keycode : Some(Keycode::Down), .. } =>
        {
          hold_down_p2 = false ;
        },
        _ => {}
      }
    }

    if game && !first_time
    {
      // paddle mvt
      if hold_up_p1 && paddle_p1_y > paddle_y_limit_min
      {
        paddle_p1_y -= paddle_speed ;
      }
      else if hold_down_p1 && paddle_p1_y < paddle_y_limit_max
      {
        paddle_p1_y += paddle_speed ;
      }

      if hold_up_p2 && paddle_p2_y > paddle_y_limit_min
      {
        paddle_p2_y -= paddle_speed ;
      }
      else if hold_down_p2 && paddle_p2_y < paddle_y_limit_max
      {
        paddle_p2_y += paddle_speed ;
      }

      // ball mvt

      // since we move at most ball_speed pixel, we check colisions at + or - ball_speed/2
      if (ball_x - ball_size/2 - (paddle_p1_x + paddle_size_w/2)).abs() <= ball_speed/2
      && ball_y + ball_size/2 >= paddle_p1_y - paddle_size_h/2
      && ball_y - ball_size/2 <= paddle_p1_y + paddle_size_h/2
      {
        if ball_y <= paddle_p1_y
        {
          angle = -(((paddle_p1_y - ball_y)/(ball_x - paddle_p1_x)) as f64).atan() ;
        }
        else
        {
          angle = (((ball_y - paddle_p1_y)/(ball_x - paddle_p1_x)) as f64).atan() ;
        }
      }
      else if (ball_x + ball_size/2 - (paddle_p2_x - paddle_size_w/2)).abs() <= ball_speed/2
      && ball_y + ball_size/2 >= paddle_p2_y - paddle_size_h/2
      && ball_y - ball_size/2 <= paddle_p2_y + paddle_size_h/2
      {
        if ball_y <= paddle_p2_y
        {
          angle = -PI + (((paddle_p2_y - ball_y)/(paddle_p2_x - ball_x)) as f64).atan() ;
        }
        else
        {
          angle = PI - (((ball_y - paddle_p2_y)/(paddle_p2_x - ball_x)) as f64).atan() ;
        }
      }
      
      if (ball_y - ball_size/2).abs() <= ball_speed/2
           || (ball_y + ball_size/2 - w_height).abs() <= ball_speed/2
      {
        angle = -angle ;
      }

      ball_x += (ball_speed as f64 * angle.cos()) as i32 ;
      ball_y += (ball_speed as f64 * angle.sin()) as i32 ;

      // score test
      if ball_x - ball_size/2 <= ball_speed/2
      {
        score_p2 += 1 ;
        game = false ;

        ball_x = w_width/2 ; 
        ball_y = w_height/2 ;
        angle = -PI ;

        paddle_p1_y = w_height/2 ;
        paddle_p2_y = w_height/2 ;
      }
      else if ball_x + ball_size/2 >= w_width - ball_speed/2
      {
        score_p1 += 1 ;
        game = false ;

        ball_x = w_width/2 ; 
        ball_y = w_height/2 ;
        angle = 0.0 ;

        paddle_p1_y = w_height/2 ;
        paddle_p2_y = w_height/2 ;
      }
    }
    else if !game
    {
      // Only to render but not starting the game bc we wait before lauching so we need
      // to render once
      if first_time 
      {
        first_time = false ;
      }
      else
      {
        ::std::thread::sleep(Duration::from_millis(2_000)) ;

        game = true ;
      }
    }

    // rendering
    draw_rec(&mut canvas, Color::RGB(200, 200, 200), ball_x - ball_size/2, ball_y - ball_size/2, ball_size, ball_size) ;
    draw_rec(&mut canvas, Color::RGB(200, 200, 200), paddle_p1_x - paddle_size_w/2, paddle_p1_y - paddle_size_h/2, paddle_size_w, paddle_size_h) ;
    draw_rec(&mut canvas, Color::RGB(200, 200, 200), paddle_p2_x - paddle_size_w/2, paddle_p2_y - paddle_size_h/2, paddle_size_w, paddle_size_h) ;

    let score_text = format!("{}     -     {}", score_p1, score_p2) ;
    let surface = font
      .render(&score_text)
      .blended(Color::RGB(200, 200, 200))
      .unwrap() ;

    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .unwrap() ;

    canvas.copy(&texture, None, Some(Rect::new(w_width/2 - surface.size().0 as i32/5, w_height/20, 2*surface.size().0/5, 2*surface.size().1/5))).expect("can't render score") ;

    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60)) ;
  } 
}