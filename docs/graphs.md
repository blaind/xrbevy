# screenshot 0
```mermaid
graph LR
    style XRD fill:#555555,stroke:#fff,stroke-width:2px
    style XRSC fill:#555555,stroke:#fff,stroke-width:2px
    style OXRS fill:#555555,stroke:#fff,stroke-width:2px
    style XRS fill:#555555,stroke:#fff,stroke-width:2px
    style EDBH fill:#555555,stroke:#fff,stroke-width:2px
    style XRO fill:#555555,stroke:#fff,stroke-width:2px
    style XRHT fill:#555555,stroke:#fff,stroke-width:2px
    style XRI2 fill:#555555,stroke:#fff,stroke-width:2px

    style ScheduleRunnerSettings fill:#003300,stroke:#000,stroke-width:2px
    style XRH fill:#003300,stroke:#000,stroke-width:2px
    style XRI fill:#003300,stroke:#000,stroke-width:2px

    style BO fill:#003355,stroke:#000,stroke-width:2px
    style BOC fill:#003355,stroke:#000,stroke-width:2px

    style s_E fill:#ff9900,stroke:#000,stroke-width:2px,color:#333
    style s_CA fill:#ff9900,stroke:#000,stroke-width:2px,color:#333

    style XRCP fill:#00ff00,stroke:#000,stroke-width:2px,color:#333

    style XRP fill:#00ff00,stroke:#000,stroke-width:2px,color:#333

    BO[bevy_openxr] --- BOC[bevy_openxr_core]

    XRD --- XRSC[XRSwapchain]
    XRSC --- Framebuffer
    BOC --- XRD[XRDevice]
    BOC --- s_E[openxr_event_system]
    XRD --- OXRS[OpenXRStruct]
    OXRS --- XRS[XRState]
    OXRS --- EDBH[EventDataBufferHolder]
    OXRS --- XRH[wgpu::OpenXRHandles]
    OXRS --- XRI[openxr::Instance]
    OXRS --- XRO[OpenXROptions]
    XRSC --- XRVT[XRViewTransform]
    XRSC --- XRHT[HandTrackers]
    XRHT --- XRHPS[HandPoseState]

    BOC --- XRCP[OpenXRCorePlugin]

    BO --- XRP[OpenXRPlugin]

    XRP --- XRS2[OpenXRSettings]
    XRP --- PS[ProjectionState]
    XRP --- ScheduleRunnerSettings
    XRP --- s_CA[openxr_camera_system]
    XRP --- XRI2[OpenXRInstance]


```

# screenshot 1

```mermaid
sequenceDiagram
    participant bevy_openxr
    participant wgpu_rs
    participant wgpu_core
    participant gfx_backend_vulkan

    bevy_openxr->>wgpu_rs: wgpu_openxr::new(.., instance, opt)
    wgpu_rs->>wgpu_core: WGPUOpenXR::configure(..,instance, opt)

    wgpu_core->>gfx_backend_vulkan: OpenXR::configure(instance)

    gfx_backend_vulkan->>gfx_backend_vulkan: initial openxr configuration
    gfx_backend_vulkan->>gfx_backend_vulkan: store instance to global Lazy<Mutex<Option<Instance>>>

    gfx_backend_vulkan->>wgpu_core: OpenXR

    wgpu_core->>wgpu_rs: WGPUOpenXR

    wgpu_rs->>bevy_openxr: WGPUOpenXR

    bevy_openxr->>bevy_openxr: call set_openxr(), which will store to global OnceCell
```

# screenshot 2

```mermaid
sequenceDiagram
    participant bevy_openxr
    participant wgpu_rs
    participant wgpu_core
    participant gfx_backend_vulkan

    bevy_openxr->>wgpu_core: call (local)wgpu_openxr.get_session_handles

    wgpu_core->gfx_backend_vulkan: call (static)get_session_handles()

    gfx_backend_vulkan->>wgpu_core: (Session<Vulkan>, FrameWaiter, FrameStream<Vulkan>, Space, SystemId)

    wgpu_core->>bevy_openxr: OpenXRHandles

```

# screenshot 3

```mermaid
graph LR
    R[RUNTIME]
    I[xrInstance] --- S[xrSession]
    S --- SC[xrSwapchain]
    V[xrView] --- FOV[xrFovf]
    V --- POSE[xrPosef]
    R --- ALP[xrApiLayerProperties]
    EBM[xrEnvironmentBlendMode]
    SID[xrSystemId]
    FF[xrFormFactor]
    SPA[xrSpace]
    S --- RST[xrReferenceSpaceType]
    SID --- VCT[xrViewConfigurationType]

    I --- AS[xrActionSet]
    AS --- A[xrAction]
    S --- HT[xrHandTrackerEXT]
    R --- I
    S --- SPA
    SID --- FF
    S --- V

    R --- EP[xrExtensionProperties]

    FEI[xrFrameEndInfo] --- EBM
    S --- FEI

    A --- P[xrPath]
    A --- AT[xrActionType]

    I --- VKI[VkInstance]


    style R fill:#00ff00,stroke:#000,stroke-width:2px,color:#555
    style I fill:#ff8800,stroke:#000,stroke-width:2px,color:#fff
    style S fill:#ff8800,stroke:#000,stroke-width:2px,color:#fff
    style SC fill:#ff8800,stroke:#000,stroke-width:2px,color:#fff

    style ALP fill:#003300,stroke:#000,stroke-width:2px
    style EP fill:#003300,stroke:#000,stroke-width:2px
    style SID fill:#003300,stroke:#000,stroke-width:2px
    style FF fill:#003300,stroke:#000,stroke-width:2px
    style VCT fill:#003300,stroke:#000,stroke-width:2px
    style EBM fill:#003300,stroke:#000,stroke-width:2px
    style FOV fill:#003300,stroke:#000,stroke-width:2px
    style POSE fill:#003300,stroke:#000,stroke-width:2px
    style RST fill:#003300,stroke:#000,stroke-width:2px


    style AS fill:#555555,stroke:#fff,stroke-width:2px
    style A fill:#555555,stroke:#fff,stroke-width:2px
    style SPA fill:#555555,stroke:#fff,stroke-width:2px
    style HT fill:#555555,stroke:#fff,stroke-width:2px

    style V fill:#003355,stroke:#000,stroke-width:2px
    style FEI fill:#003355,stroke:#000,stroke-width:2px

    style VKI fill:#ffff00,stroke:#000,stroke-width:2px,color:#333

```

