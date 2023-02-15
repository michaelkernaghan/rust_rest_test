#[cfg(test)]
mod tests {

    const POST_URL: &str = "http://127.0.0.1:5001/apdu";
    const BUTTON_BOTH: &str = "http://127.0.0.1:5001/button/both";
    const BUTTON_RIGHT: &str = "http://127.0.0.1:5001/button/right";
    const PRELIM_REQUEST: &str = "8004000011048000002c800006c18000000080000000";
    const EVENTS_URL: &str = "http://127.0.0.1:5001/events";

    #[test]
    fn increase_paid_storage_operation() {

        let payload_request = "800481005703d4e4d777724188dadeac0c3817efb0cf430a0d5ea8aa6883b3176aa2b59fc5ae71004035f49a9d068f852084ddf642835bbfdd4ff681de02fbff02e807009c01014bab7e94bd5ee22b2f65dfb9955981df9aaad3b400";

        let _res: ureq::Response = ureq::delete(EVENTS_URL).call().unwrap();

        let res: ureq::Response = ureq::post(POST_URL)
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({ "data": PRELIM_REQUEST }))
            .unwrap();
        assert!(res.into_string().unwrap().contains("9000"));

        let res2: ureq::Response = ureq::post(POST_URL)
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({ "data": payload_request }))
            .unwrap();

        for _clicks in 1..13 {
            let _res: ureq::Response = ureq::post(BUTTON_RIGHT)
                .set("Content-Type", "application/json")
                .send_json(ureq::json!({
                    "action":"press-and-release"
                }))
                .unwrap();
        }

        let _res: ureq::Response = ureq::post(BUTTON_BOTH)
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({
                "action":"press-and-release"
            }))
            .unwrap();
        assert!(res2.into_string().unwrap().contains("9000"))
    }
}
