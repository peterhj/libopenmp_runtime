extern crate libc;

use libc::*;

// XXX: No explicit linkage.
extern "C" {
  pub fn omp_get_thread_limit() -> c_int;
  pub fn omp_get_thread_num() -> c_int;
  pub fn omp_get_num_procs() -> c_int;
  pub fn omp_get_num_threads() -> c_int;
  pub fn omp_set_num_threads(num_threads: c_int);
}
