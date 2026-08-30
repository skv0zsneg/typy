use typy::object::Object;

#[test]
fn adds_integers() {
    let a = Object::Int(5);
    let b = Object::Int(3);
    assert_eq!(a.add(&b).unwrap(), Object::Int(8));
}

#[test]
fn subtracts_integers() {
    let a = Object::Int(10);
    let b = Object::Int(4);
    assert_eq!(a.sub(&b).unwrap(), Object::Int(6));
}

#[test]
fn multiplies_integers() {
    let a = Object::Int(6);
    let b = Object::Int(7);
    assert_eq!(a.mul(&b).unwrap(), Object::Int(42));
}

#[test]
fn divides_integers() {
    let a = Object::Int(20);
    let b = Object::Int(4);
    assert_eq!(a.div(&b).unwrap(), Object::Int(5));
}

#[test]
fn rejects_division_by_zero() {
    let a = Object::Int(10);
    let b = Object::Int(0);
    let result = a.div(&b);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("ZeroDivisionError"));
}

#[test]
fn rejects_arithmetic_with_bool() {
    let a = Object::Int(5);
    let b = Object::Bool(true);
    assert!(a.add(&b).is_err());
    assert!(a.sub(&b).is_err());
    assert!(a.mul(&b).is_err());
    assert!(a.div(&b).is_err());
}

#[test]
fn compares_integers() {
    let a = Object::Int(5);
    let b = Object::Int(10);

    assert!(a.lt(&b).unwrap());
    assert!(!a.gt(&b).unwrap());
    assert!(a.le(&b).unwrap());
    assert!(!a.ge(&b).unwrap());
}

#[test]
fn checks_equality() {
    let a = Object::Int(5);
    let b = Object::Int(5);
    let c = Object::Int(10);

    assert!(a.eq(&b).unwrap());
    assert!(!a.eq(&c).unwrap());

    let t1 = Object::Bool(true);
    let t2 = Object::Bool(true);
    let f = Object::Bool(false);

    assert!(t1.eq(&t2).unwrap());
    assert!(!t1.eq(&f).unwrap());
}

#[test]
fn checks_inequality() {
    let a = Object::Int(5);
    let b = Object::Int(10);

    assert!(a.ne(&b).unwrap());
    assert!(!a.ne(&a).unwrap());
}

#[test]
fn rejects_comparison_of_different_types() {
    let a = Object::Int(5);
    let b = Object::Bool(true);

    assert!(a.eq(&b).is_err());
    assert!(a.ne(&b).is_err());
    assert!(a.lt(&b).is_err());
}

#[test]
fn displays_correctly() {
    assert_eq!(format!("{}", Object::Int(42)), "42");
    assert_eq!(format!("{}", Object::Bool(true)), "True");
    assert_eq!(format!("{}", Object::Bool(false)), "False");
    assert_eq!(format!("{}", Object::None), "None");
}

#[test]
fn type_names_are_correct() {
    assert_eq!(Object::Int(0).type_name(), "int");
    assert_eq!(Object::Bool(false).type_name(), "bool");
    assert_eq!(Object::None.type_name(), "NoneType");
}
