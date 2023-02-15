
   /* 
    Origination Operation
    Example of how to automate calls to Speculos REST API
     */
fn main() {
    let post_url = "http://127.0.0.1:5001/apdu";
    let button_both = "http://127.0.0.1:5001/button/both";
    let button_right = "http://127.0.0.1:5001/button/right";

    let url = "http://127.0.0.1:5001/events";
    let res: ureq::Response = ureq::delete(url).call().unwrap();
    println!("HTTP GET, no path interpolation, no query parameters:\n- URL: {}\n- res code: {},\n- res body:\n{}",
    url,
    res.status(),
    res.into_string().unwrap());

    let res: ureq::Response = ureq::post(post_url)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "data": "8004000011048000002c800006c18000000080000000"
        }))
        .unwrap();
    println!(
        "HTTP POST:\n- URL: {}\n- res code: {},\n- res body:\n{}",
        post_url,
        res.status(),
        res.into_string().unwrap()
    );

    let res2: ureq::Response = ureq::post(post_url)
    .set("Content-Type", "application/json")
    .send_json(ureq::json!({
        "data": "800481005903e0097782aa4c63ce80ed4f08245e30e71a10740a79949f44ae55c5b5342dc0466c004035f49a9d068f852084ddf642835bbfdd4ff681c704feff02e9190080897a01a4fb910b2a144949612fda5917229d2ba0dfbfef0000"
    })).unwrap();

    for _clicks in 1..14 {
        let _res: ureq::Response = ureq::post(button_right)
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({
                "action":"press-and-release"
            }))
            .unwrap();
    }

    let _res: ureq::Response = ureq::post(button_both)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "action":"press-and-release"
        }))
        .unwrap();
    println!(
        "HTTP POST:\n- URL: {}\n- res code: {},\n- res body:\n{}",
        post_url,
        res2.status(),
        res2.into_string().unwrap()
    );
}
