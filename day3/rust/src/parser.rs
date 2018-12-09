use nom::types::CompleteStr;
use super::rect::Rect;

fn is_numeric(c: char) -> bool {
  c >= '0' && c <= '9'
}

fn map_to_number(s: CompleteStr) -> Result<u16, std::num::ParseIntError> {
  s.parse()
}

named!(get_num<CompleteStr, u16>,
  map_res!(take_while!(is_numeric), map_to_number)
);

named!(get_id<CompleteStr, u16>,
  do_parse!(
    tag!("#")    >>
    id: get_num  >>
    (id)
  )
);

named!(get_position<CompleteStr, (u16, u16)>,
  do_parse!(
    left: get_num >>
    tag!(",")     >>
    top: get_num  >>
    ((left, top))
  )
);

named!(get_size<CompleteStr, (u8, u8)>,
  do_parse!(
    width: get_num  >>
    tag!("x")       >>
    height: get_num >>
    ((width as u8, height as u8))
  )
);

named!(rect_def<CompleteStr, Rect>,
  do_parse!(
    id: get_id            >>
    tag!(" @ ")           >>
    position: get_position >>
    tag!(": ")            >>
    size: get_size        >>
    (Rect { id, top: position.1, left: position.0, width: size.0, height: size.1 })
  )
);

pub fn parse(input: &str) -> Result<Rect, nom::Err<CompleteStr>> {
  match rect_def(CompleteStr::from(input)) {
    Ok((_, r)) => Result::Ok(r),
    Err(err) => Result::Err(err)
  }
}