pub fn is_leap_year(y: i64) ->bool {
    if y % 100 == 0 {
        return y % 400 == 0
    }

    return y % 4 == 0
}