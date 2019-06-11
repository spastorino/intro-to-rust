// Exercise 1: Define Status with Single and Married variants.
// In case of Married, variant has the name (String) of the partner.

// Exercise 2: Define SchoolDegree with at least HighSchool variant.

// Exercise 3: Define Person with:
// name of type String
// marital_status of type Status (defined above)
// school_degree of type SchoolDegree

impl Person {
    // Exercise 4: Implement a new associated function that returns a new instance of Person

    // Exercise 5: Implement partner method

    /// Returns None in case the person has no partner.
    /// Returns Some(partner_name) in case the person has a partner.
    /// You will probably get 2 compile errors here, do exactly what the compiler help tells you
    /// until we learn what's going on :).
    fn partner(&self) -> Option<String> {
    }

    // Exercise 6: Define a method that returns true if the person completed high school
    // If you try to use == you would need to deal with PartialEq trait, you can just match.
}

fn main() {
    // Exercise 7: Build a married person
}
