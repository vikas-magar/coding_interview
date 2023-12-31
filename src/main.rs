mod ae;
mod lc;

use ae::array_of_product;
use ae::first_duplicate_value;
use ae::longest_peak;
use ae::monotonic_array;
use ae::move_element_end;
use ae::smallest_diff;

use lc::astroid_collusion;

fn main() {
    smallest_diff::sd();
    move_element_end::run();
    monotonic_array::run();
    longest_peak::run();

    array_of_product::run();

    first_duplicate_value::run();
    astroid_collusion::run();
}
