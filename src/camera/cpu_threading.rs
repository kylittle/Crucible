use std::{
    sync::{Arc, Mutex, RwLock, mpsc},
    thread::{self, JoinHandle},
};

use dashmap::DashMap;
use indicatif::ProgressBar;

use crate::{camera::Camera, objects::Hittables, scene::Skybox, utils::Color};

/// Contains information to be sent to a thread
/// at runtime
pub struct ThreadInfo {
    i: u32,
    j: u32,
}

impl ThreadInfo {
    pub(super) fn new(i: u32, j: u32) -> ThreadInfo {
        ThreadInfo { i, j }
    }
}

impl Camera {
    pub(super) fn thread_setup(
        &self,
        skybox: &Skybox,
        world: &Hittables,
    ) -> (Vec<JoinHandle<()>>, Option<mpsc::Sender<ThreadInfo>>) {
        // rendering environment

        let arc_skybox = Arc::new(skybox.clone());
        let arc_cam = Arc::new(self.clone());

        // Channels
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // start threads
        let mut threads = Vec::with_capacity(self.thread_count);

        for id in 0..self.thread_count {
            // This is bad for efficiency but since each thread can have different times they all need a copy of the world
            let clone_world = world.clone();

            // Make progress bar for thread
            let work = (self.viewport.image_height * self.viewport.image_width) as u64
                / self.thread_count as u64;
            let pb = self.mp.add(ProgressBar::new(work));
            pb.set_style(self.sty.clone());

            // Start the thread
            threads.push(start_thread_cpu(
                pb,
                id,
                Arc::clone(&receiver),
                Arc::clone(&self.results),
                Arc::clone(&arc_cam),
                Arc::clone(&arc_skybox),
                Arc::new(RwLock::new(clone_world)),
            ));
        }

        (threads, Some(sender))
    }
}

// starts threads for the cpu-based renderer
pub fn start_thread_cpu(
    pb: ProgressBar,
    id: usize,
    receiver: Arc<Mutex<mpsc::Receiver<ThreadInfo>>>,
    results: Arc<DashMap<(u32, u32), Color>>,
    cam: Arc<Camera>,
    skybox: Arc<Skybox>,
    world: Arc<RwLock<Hittables>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let id = id;
        let mut progress = 0;

        let cam = Box::new(cam);
        let mut world = Box::new(world);

        loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(info) => {
                    let thread_loc_i = info.i;
                    let thread_loc_j = info.j;

                    let color = cam.cast_ray(
                        thread_loc_i,
                        thread_loc_j,
                        cam.max_depth,
                        &skybox,
                        Arc::get_mut(&mut world).unwrap().get_mut().unwrap(),
                    );

                    results.insert((thread_loc_i, thread_loc_j), color);
                    if progress % 10 == 0 {
                        pb.set_message(format!("t{id}"));
                        pb.inc(10);
                    }
                    progress += 1;
                }
                Err(_) => {
                    pb.finish_and_clear();
                    break;
                }
            }
        }
    })
}
