use gl;

pub fn blend(gl: &gl::Gl, enable: bool)
{
    unsafe {
        static mut CURRENTLY_ENABLED: bool = false;
        if enable != CURRENTLY_ENABLED
        {
            if enable
            {
                gl.Enable(gl::BLEND);
            }
            else {
                gl.Disable(gl::BLEND);
            }
            CURRENTLY_ENABLED = enable;
        }
    }
}

#[derive(PartialEq)]
pub enum CullType {
    NONE,
    BACK,
    FRONT
}

pub fn cull(gl: &gl::Gl, cull_type: CullType)
{
    unsafe {
        static mut CURRENT: CullType = CullType::NONE;
        if cull_type != CURRENT
        {
            match cull_type {
                CullType::NONE => {
                    gl.Disable(gl::CULL_FACE);
                },
                CullType::BACK => {
                    gl.Enable(gl::CULL_FACE);
                    gl.CullFace(gl::BACK);
                },
                CullType::FRONT => {
                    gl.Enable(gl::CULL_FACE);
                    gl.CullFace(gl::FRONT);
                }
            }
            CURRENT = cull_type;
        }
    }
}

pub fn depth_test(gl: &gl::Gl, enable: bool)
{
    unsafe {
        static mut CURRENTLY_ENABLED: bool = false;
        if enable != CURRENTLY_ENABLED
        {
            if enable
            {
                gl.Enable(gl::DEPTH_TEST);
            }
            else {
                gl.Disable(gl::DEPTH_TEST);
            }
            CURRENTLY_ENABLED = enable;
        }
    }
}

pub fn depth_write(gl: &gl::Gl, enable: bool)
{
    unsafe {
        static mut CURRENTLY_ENABLED: bool = true;
        if enable != CURRENTLY_ENABLED
        {
            if enable
            {
                gl.DepthMask(gl::TRUE);
            }
            else {
                gl.DepthMask(gl::FALSE);
            }
            CURRENTLY_ENABLED = enable;
        }
    }
}