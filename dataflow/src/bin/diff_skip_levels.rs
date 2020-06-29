// Example from: https://timelydataflow.github.io/differential-dataflow/introduction.html

use differential_dataflow::input::InputSession;
use differential_dataflow::operators::Join;

fn main() {
    // define a new timely dataflow computation
    timely::execute_from_args(std::env::args(), move |worker| {
        // create an input collection of data
        let mut input = InputSession::new();

        // define a new computation
        let probe = worker.dataflow(|scope| {
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
                .probe()
        });

        // Read a size for our organization from the arguments
        let size: usize = std::env::args()
            .nth(1)
            .expect("size: pass an numeric argument")
            .parse()
            .expect("size: argument must be an integer");

        // Step 1: Load input (a binary tree)
        input.advance_to(0); // Sets the time to t=0
        let mut person = worker.index();
        while person < size {
            input.insert((person/2, person));
            person += worker.peers();
        }

        // Wait for data loading to complete
        input.advance_to(1);
        input.flush();
        while probe.less_than(input.time()) {
            worker.step();
        }
        println!("{:?}\tdata loaded", worker.timer().elapsed());

        // Step 2: make changes to the org structure
        let mut person = 1 + worker.index();
        while person < size {
            // Sets the time to t=t+1
            input.remove((person/2, person));
            input.insert((person/3, person));
            input.advance_to(person);
            input.flush();
            while probe.less_than(input.time()) {
                worker.step();
            }
            println!("{:?}\tstep {} complete", worker.timer().elapsed(), person);
            person += worker.peers();
        }

    }).expect("computation terminated abnormally");
}
