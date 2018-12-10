use nom::types::CompleteStr;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Log {
  pub timestamp: NaiveDateTime,
  pub payload: LogPayload
}

#[derive(Debug)]
pub enum LogPayload {
  BeginsShift(u32),
  FallsAsleep,
  WakesUp
}

fn is_date_like(c: char) -> bool {
  is_digit(c) || c == '-' || c == ' ' || c == ':'
}

fn is_digit(c: char) -> bool {
  c >= '0' && c <= '9'
}

named!(get_timestamp<CompleteStr, NaiveDateTime>,
  do_parse!(
    tag!("[") >>
    timestamp_str: take_while!(is_date_like) >>
    tag!("]") >>
    (NaiveDateTime::parse_from_str(format!("{}", timestamp_str).as_str(), "%Y-%m-%d %H:%M").unwrap())
  )
);

named!(get_number<CompleteStr, u32>,
  do_parse!(number: take_while!(is_digit) >> (number.parse::<u32>().unwrap()))
);

named!(get_id<CompleteStr, u32>,
  do_parse!(
    tag!("#") >>
    number: get_number >>
    (number)
  )
);

named!(get_begin<CompleteStr, LogPayload>,
  do_parse!(
    tag!("Guard ") >>
    id: get_id >>
    tag!(" begins shift") >>
    (LogPayload::BeginsShift(id))
  )
);

named!(get_wake<CompleteStr, LogPayload>,
  do_parse!(
    tag!("wakes up") >>
    (LogPayload::WakesUp)
  )
);

named!(get_asleep<CompleteStr, LogPayload>,
  do_parse!(
    tag!("falls asleep") >>
    (LogPayload::FallsAsleep)
  )
);

named!(get_payload<CompleteStr, LogPayload>,
  alt!(get_begin | get_wake | get_asleep)
);

named!(get_log<CompleteStr, Log>,
  do_parse!(
    timestamp: get_timestamp >>
    tag!(" ") >>
    payload: get_payload >>
    (Log { timestamp, payload })
  )
);

pub fn parse_log(log: &str) -> Result<Log, nom::Err<CompleteStr>> {
  let (_, parsed_log) = get_log(CompleteStr::from(log))?;
  Result::Ok(parsed_log)
}