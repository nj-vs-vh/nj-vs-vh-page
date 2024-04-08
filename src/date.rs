use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Date {
    pub year: u16,
    pub month: Option<u16>,
    pub day: Option<u16>,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if let Some(d) = self.day {
            parts.push(d.to_string());
        }
        if let Some(m) = self.month {
            parts.push(
                match m {
                    1 => "jan",
                    2 => "feb",
                    3 => "mar",
                    4 => "apr",
                    5 => "may",
                    6 => "jun",
                    7 => "jul",
                    8 => "aug",
                    9 => "sep",
                    10 => "oct",
                    11 => "nov",
                    12 => "dec",
                    _ => "",
                }
                .to_owned(),
            );
        }
        parts.push(self.year.to_string());
        f.write_str(&parts.join(" "))
    }
}

impl std::cmp::Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (cmp_set_unset, cmp_unset_set) =
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less);
        if self.year != other.year {
            return self.year.cmp(&other.year);
        }
        if let Some(self_month) = self.month {
            if let Some(other_month) = other.month {
                if self_month != other_month {
                    return self_month.cmp(&other_month);
                }
                if let Some(self_day) = self.day {
                    if let Some(other_day) = other.day {
                        return self_day.cmp(&other_day);
                    } else {
                        return cmp_set_unset;
                    }
                } else {
                    return if other.day.is_none() {
                        std::cmp::Ordering::Equal
                    } else {
                        cmp_unset_set
                    };
                }
            } else {
                return cmp_set_unset;
            }
        } else {
            return if other.month.is_none() {
                std::cmp::Ordering::Equal
            } else {
                cmp_unset_set
            };
        }
    }
}

impl std::cmp::PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
