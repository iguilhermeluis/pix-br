#[cfg(test)]
mod tests {
    use pix_br::pix;
    #[test]
    fn some_data() {
        let pix_demo = "00020101021126720014br.gov.bcb.pix0121pix@guilhermeluis.com02251x Havaina, 2x Caipirinha520400005303986540550.725802BR5923Guilherme Luis Faustino6014Rio de Janeiro62070503***6304C644";
        let brcode = pix::brcode(
            "pix@guilhermeluis.com",           //key (required)
            "Guilherme Luis Faustino",         //name (required)
            Some(50.72),                       //amount (optional)
            Some("Rio de Janeiro"),            //city (required)
            Some("1x Havaina, 2x Caipirinha"), //description (optional)
            None,                              //zip_code (optional)
            None,                              //is_unique_transaction (optional)
        );
        assert_eq!(pix_demo, brcode);
    }

    #[test]
    fn empty_amount() {
        let pix_demo = "00020101021126430014br.gov.bcb.pix0121pix@guilhermeluis.com5204000053039865802BR5923Guilherme Luis Faustino6014Rio de Janeiro62070503***6304EC03";
        let brcode = pix::brcode(
            "pix@guilhermeluis.com",   //key (required)
            "Guilherme Luis Faustino", //name (required)
            None,                      //amount (optional)
            Some("Rio de Janeiro"),    //city (required)
            None,                      //description (optional)
            None,                      //zip_code (optional)
            None,                      //is_unique_transaction (optional)
        );
        assert_eq!(pix_demo, brcode);
    }
    #[test]
    fn full_data() {
        let pix_demo = "00020101021126870014br.gov.bcb.pix0111115633606300250Lorem Ipsum is simply dummy text of the printing a520400005303986540550.725802BR5908John doe6015Mato Grosso do 61087908104162070503***63040976";
        let brcode = pix::brcode(
            "11563360630",              //key (required)
            "John doe",                 //name (required)
            Some(50.72),                //amount (optional)
            Some("Mato Grosso do Sul"), //city (required)
            Some("Lorem Ipsum is simply dummy text of the printing and typesetting industry. when an unknow"), //description (optional)
            Some("79081041"),           //zip_code (optional)
            Some(false),                       //is_unique_transaction (optional)
        );
        assert_eq!(pix_demo, brcode);
    }
}
