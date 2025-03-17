// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        //컴파일러는 255 값이 i8에 들어갈 수 없다는 것을 충분히 똑똑하게 알고 있기 때문에 하드 에러를 발생시킵니다.
        // 이 (잘못된) 변환을 가능하게 하기 위해 의도적으로 이 가드레일을 비활성화했습니다.
        // 컴파일러는 값이 리터럴이기 때문에 이를 선택할 수만 있습니다.
        // 만약 변수를 사용한다면 컴파일러는 컴파일 타임에 이를 포착할 수 없습니다.
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        //위와 똑같은 식을 사용하여 이 문제를 해결할 수도 있지만, 그렇게 하면 연습의 목적에 어긋납니다.
        // 대신 `u8`로 변환하면 `255`에 해당하는 진짜 `i8` 값을 사용하십시오.
        let y: i8 = -1;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
