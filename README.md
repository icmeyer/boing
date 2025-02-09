1/24/2025

boing is a simple rust demonstration of a ball bouncing off walls

Using bevy for the graphics so that I can focus on physics 😃

Here are the features (in rough order of implementation):
- sphere and rectangle object implementations
- inelastic collision between objects
    - Collision detection
        - Axes
        - Vertices from axes
        - Shape projection onto axes
        - separating axis theorem
- gravity between objects
- wgpu visualization of the scene
- UPS/FPS physics manager
- WASM for running online
