pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}

pub struct Servers<'a> {
    pub nodes: Vec<Server<'a>>,
}

impl<'a> Servers <'a> {
    pub fn new() -> Servers<'a> {
        Servers {
            nodes: vec![  
                        Server {
                            name: "NorthAmerica-1",
                            location: "New York City",
                            coords: (40.71, -74.00),
                            status: "Up",
                        },
                        Server {
                            name: "Europe-1",
                            location: "Paris",
                            coords: (48.85, 2.35),
                            status: "Failure",
                        },
                        Server {
                            name: "SouthAmerica-1",
                            location: "São Paulo",
                            coords: (-23.54, -46.62),
                            status: "Up",
                        },
                        Server {
                            name: "Asia-1",
                            location: "Singapore",
                            coords: (1.35, 103.86),
                            status: "Up",
                        },
                    ],
        }
    }

}  
