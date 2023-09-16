mod functions;

fn main() {

    // exercise 1
    println!(">> 1");
    println!("heaviside");
    println!("-20: {}", functions::heaviside(-20.0));
    println!("5: {}", functions::heaviside(5.0));
    println!("abs");
    println!("-20: {}", functions::absolute_val(-20.0));
    println!("5: {}", functions::absolute_val(5.0));
    println!("sng");
    println!("-20: {}", functions::sng(-20.0));
    println!("5: {}", functions::sng(5.0));
    println!("floor");
    println!("-20.0023: {}", functions::floor(-20.0023));
    println!("5.847239472: {}", functions::floor(5.847239472));
    println!("mantissa");
    println!("-20.0023: {}", functions::mantissa(-20.0023));
    println!("5.847239472: {}", functions::mantissa(5.847239472));

    // exercise 2
    println!(">> 2");
    let (triangle_perimeter, trangle_area) = functions::is_trangle(4.0, 5.0, 3.0);
    println!("triangle: {} {}", triangle_perimeter, trangle_area);

    // exercise 3
    println!(">> 3");
    println!("point (0,0) in triangle: {}", functions::is_in_trangle( (0.0, 0.0) ));
    println!("point (-7,0) in triangle: {}", functions::is_in_trangle( (-7.0, 0.0) ));
    println!("point (1,1) in triangle: {}", functions::is_in_trangle( (1.0, 1.0) ));

    // exercise 4
    println!(">> 4");
    println!("point (2,2) in triangle: {}", functions::get_number_of_quadrant_of_cartesian( 2.0, 2.0 ));
    println!("point (2,-2) in triangle: {}", functions::get_number_of_quadrant_of_cartesian( 2.0, -2.0 ));
    println!("point (-2,2) in triangle: {}", functions::get_number_of_quadrant_of_cartesian( -2.0, 2.0 ));
    println!("point (-2,-2) in triangle: {}", functions::get_number_of_quadrant_of_cartesian( -2.0, -2.0 ));

    // exercise 5
    println!(">> 5");
    let bite_a:i32 = 2;     // Bit presentation 10
    let bite_b:i32 = 3;     // Bit presentation 11
    println!("bit values: a, b : {} {}", bite_a, bite_b);

    println!("(a & b) => {} ", bite_a & bite_b);
    println!("(a | b) => {} ", bite_a | bite_b) ;
    println!("(a ^ b) => {} ", bite_a ^ bite_b);
    println!("(!b) => {} ", !bite_b);
    println!("(a << b) => {}", bite_a << bite_b);
    println!("(a >> b) => {}", bite_a >> bite_b);

    // exercise 6
    println!(">> 6");
    let s: &str = "Ala ma kota";
    let s_vec = functions::split_sentence_into_words(s);
    println!("{}-{}-{}", s_vec[0], s_vec[1], s_vec[2]);

    // exercise 7
    println!(">> 7");
    println!("PESEL");

    // exercise 8
    println!(">> 8");
    println!("{}: bin {:b}, hex {:x}, oct {:o}", bite_b, bite_b, bite_b, bite_b);


}
