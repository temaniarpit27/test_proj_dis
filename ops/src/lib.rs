use paladin::registry;
use paladin::{
    operation::{Monoid, Operation, Result},
    RemoteExecute,
};
use serde::{Deserialize, Serialize};

registry!();

#[derive(Serialize, Deserialize, RemoteExecute)]
pub struct FibAt1;

impl Operation for FibAt1 {
    type Input = u64;
    type Output = u64;

    fn execute(&self, input: Self::Input) -> Result<Self::Output> {
        match input {
            0 => Ok(0),
            1 => Ok(1),
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=input {
                    let temp = a;
                    a = b;
                    b = temp + b;
                }
                Ok(b)
            }
        }
    }
}

#[derive(Serialize, Deserialize, RemoteExecute)]
pub struct FibAt2;

impl Operation for FibAt2 {
    type Input = u64;
    type Output = u64;

    fn execute(&self, input: Self::Input) -> Result<Self::Output> {
        match input {
            0 => Ok(0),
            1 => Ok(1),
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=input {
                    let temp = a;
                    a = b;
                    b = temp + b;
                }
                Ok(b)
            }
        }
    }
}

// Define a Sum monoid.
#[derive(Serialize, Deserialize, RemoteExecute)]
pub struct Sum;

impl Monoid for Sum {
    type Elem = u64;

    fn combine(&self, a: Self::Elem, b: Self::Elem) -> Result<Self::Elem> {
        Ok(a + b)
    }

    fn empty(&self) -> Self::Elem {
        0
    }
}

// Define a Sum monoid.
#[derive(Serialize, Deserialize, RemoteExecute)]
pub struct Mul;

impl Monoid for Mul {
    type Elem = u64;

    fn combine(&self, a: Self::Elem, b: Self::Elem) -> Result<Self::Elem> {
        Ok(a * b)
    }

    fn empty(&self) -> Self::Elem {
        0
    }
}
