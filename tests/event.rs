#[cfg(test)]
mod tests {

    const POST_URL: &str = "http://127.0.0.1:5001/apdu";
    const BUTTON_BOTH: &str = "http://127.0.0.1:5001/button/both";
    const BUTTON_RIGHT: &str = "http://127.0.0.1:5001/button/right";
    const PRELIM_REQUEST: &str = "8004000011048000002c800006c18000000080000000";
    const EVENTS_URL: &str = "http://127.0.0.1:5001/events";

    #[test]
    fn event_operation() {
        let payload_request = "800481005903e0097782aa4c63ce80ed4f08245e30e71a10740a79949f44ae55c5b5342dc0466c004035f49a9d068f852084ddf642835bbfdd4ff681c704feff02e9190080897a01a4fb910b2a144949612fda5917229d2ba0dfbfef0000";

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

        for _clicks in 1..14 {
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
