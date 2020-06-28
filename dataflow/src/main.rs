use differential_dataflow::input::InputSession;
use differential_dataflow::operators::Join;

fn main() {
    // define a new timely dataflow computation
    timely::execute_from_args(std::env::args(), move |worker| {
        // create an input collection of data
        let mut input = InputSession::new();

        // define a new computation
        worker.dataflow(|scope| {
            // create a new collection from our input
            let manages = input.to_collection(scope);

            // if (m2, m1) and (m1, p), then output (m1, (m2, p))
            // i.e. for each pair in the join where the report in pair 1 is the manager in pair 2
            // output (person, (person's manager, person's report))
            manages
                // .inspect(|(data, time, diff)| println!("(m, r): time={:?}, diff={:?}, data={:?}", time, diff, data))
                .map(|(manager, report)| (report, manager)) // reverse so the report becomes the "key" for the join
                // .inspect(|(data, time, diff)| println!("(r, m): time={:?}, diff={:?}, data={:?}", time, diff, data))
                .join(&manages)
                // .inspect(|(data, time, diff)| println!("(p, (m, r)): time={:?}, diff={:?}, data={:?}", time, diff, data))
                ;
        });

        // Read a size for our organization from the arguments
        let size: u32 = std::env::args()
            .nth(1)
            .expect("size: pass an numeric argument")
            .parse()
            .expect("size: argument must be an integer");

        // Step 1: Load input (a binary tree)
        input.advance_to(0); // Sets the time to t=0
        for person in 0..size {
            input.insert((person/2, person));
        }

        // Step 2: make changes to the org structure
        for person in 1..size {
            // Sets the time to t=t+1
            input.advance_to(person);
            input.remove((person/2, person));
            input.insert((person/3, person));
        }

    }).expect("computation terminated abnormally");
}
