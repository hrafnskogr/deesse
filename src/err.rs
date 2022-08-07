/*
 * Error module
 * For custom errors definition
 */

use std::error::Error;
use std::fmt;


pub struct DSErr
{
    pub message: String,
}

impl fmt::Display for DSErr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for DSErr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.message)
    }
}

impl Error for DSErr {}
