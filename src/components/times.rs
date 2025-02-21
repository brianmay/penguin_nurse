pub fn div_rem(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

pub fn time_delta_to_string(duration: chrono::TimeDelta) -> String {
    let seconds = duration.num_seconds();
    let (negative, seconds) = if seconds < 0 {
        (true, -seconds)
    } else {
        (false, seconds)
    };
    let (minutes, seconds) = div_rem(seconds, 60);
    let (hours, minutes) = div_rem(minutes, 60);
    let (days, hours) = div_rem(hours, 24);

    let negative_string = if negative { "negative " } else { "" };

    if duration.num_seconds().abs() < 60 {
        format!("{negative_string}{seconds} seconds")
    } else if duration.num_minutes().abs() < 60 {
        format!("{negative_string}{minutes} minutes + {seconds} seconds")
    } else if duration.num_hours().abs() < 24 {
        format!("{negative_string}{hours} hours + {minutes} minutes")
    } else {
        format!("{negative_string}{days} days + {hours} hours")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_rem() {
        assert_eq!(div_rem(10, 3), (3, 1));
        assert_eq!(div_rem(10, 2), (5, 0));
        assert_eq!(div_rem(10, 1), (10, 0));
        assert_eq!(div_rem(10, 5), (2, 0));
        assert_eq!(div_rem(10, 4), (2, 2));
    }

    #[test]
    fn test_time_delta_to_string() {
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::seconds(10)),
            "10 seconds"
        );
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::seconds(-10)),
            "negative 10 seconds"
        );
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::minutes(10)),
            "10 minutes + 0 seconds"
        );
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::minutes(-10)),
            "negative 10 minutes + 0 seconds"
        );
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::hours(10)),
            "10 hours + 0 minutes"
        );
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::hours(-10)),
            "negative 10 hours + 0 minutes"
        );
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::days(10)),
            "10 days + 0 hours"
        );
        assert_eq!(
            time_delta_to_string(chrono::TimeDelta::days(-10)),
            "negative 10 days + 0 hours"
        );
    }
}
