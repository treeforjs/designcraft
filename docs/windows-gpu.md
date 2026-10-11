# Windows graphics setup

This guide targets **Windows 11 x64**, prioritizing **NVIDIA RTX PRO 4000 Blackwell** and
**RTX PRO 2000 Blackwell**, plus GeForce RTX 30/40/50-series cards and Intel integrated graphics.
The driver guidance comes from the hardware vendors. Windows execution and these physical GPUs
have not been validated in the Ubuntu compatibility test environment.

## Choose the driver for your GPU

| GPU | Driver choice |
|---|---|
| RTX PRO 4000 / 2000 Blackwell | Current NVIDIA RTX Enterprise Production Branch for the exact desktop or laptop model and Windows 11 64-bit |
| GeForce RTX 30 / 40 / 50 series | NVIDIA Studio Driver for creative workloads; Game Ready is also an option when gaming updates are the priority |
| Intel integrated graphics | The PC maker's graphics driver first; Intel Driver & Support Assistant can identify the appropriate generic package for the processor/graphics generation |

Use the [official NVIDIA driver selector](https://www.nvidia.com/en-us/drivers/), and check the
selected download's supported-products list before installation. The verified
[RTX Enterprise list](https://www.nvidia.com/en-us/drivers/details/279889/) includes both priority
workstation cards; the [Studio list](https://www.nvidia.com/en-us/drivers/details/280101/) includes
GeForce RTX 30/40/50 families. These establish coverage, not a minimum or pinned driver version.
NVIDIA explains [Studio versus Game Ready](https://www.nvidia.com/en-us/geforce/news/ces-2022-nvidia-community-qa/).

For Intel, follow its [OEM-first recommendation](https://www.intel.com/content/www/us/en/support/articles/000058295/graphics.html)
and [driver-generation / Driver & Support Assistant guidance](https://www.intel.com/content/www/us/en/support/articles/000090440/graphics.html).
Arc/Core Ultra, earlier Iris Xe/UHD generations and older integrated graphics use different
driver branches; choose the one covering the actual processor.

Install Windows updates and the selected vendor driver, then reboot before testing DesignCraft.
On a hybrid laptop, keep both the integrated and discrete GPU drivers current. Use the normal
signed Windows driver package for the exact GPU and OS.

## Check the adapter DesignCraft uses

DesignCraft uses **wgpu** to present the window and draw the interface. Page composition and
rasterization use the CPU (`vello_cpu`); installing CUDA is not required. Startup prefers a
high-performance adapter that can present to the current window.

The startup log records `graphics adapter`, its backend and the driver in
`%APPDATA%\DesignCraft\logs\designcraft.log`. NVIDIA users can also run `nvidia-smi`; Windows Device Manager
shows the installed display adapters and driver versions.
Run `dxdiag` and inspect the Display/Render tabs to record the GPU and driver versions. If
Windows lists [Microsoft Basic Display Adapter](https://support.microsoft.com/en-us/windows/hardware/display-graphics/microsoft-basic-display-adapter-in-windows),
install the proper vendor or OEM driver before evaluating graphics performance.

Windows also exposes a per-app preference in **Settings → System → Display → Graphics →
DesignCraft → Options** ([Microsoft instructions](https://support.microsoft.com/en-us/windows/hardware/display-graphics/optimizations-for-windowed-games-in-windows-11)).
Save changes and restart the app, then check the startup log to verify the chosen adapter.

The Windows build supports wgpu's Direct3D 12, Vulkan and OpenGL backends. Automatic selection
uses the installed adapters and drivers; the startup log identifies the actual backend.
`WGPU_POWER_PREF=low` requests a low-power adapter; the default is `high`. These are preferences,
not a promise to select a particular physical card.

If the window is black or fails to start after installing the current driver, try an explicit backend from PowerShell. Start with Direct3D 12;
use `gl` instead of `dx12` to diagnose an issue through OpenGL. This example uses the default x64 installation location; adjust it
for a portable or custom installation:

```powershell
$env:WGPU_BACKEND = "dx12"
& "$env:ProgramFiles\DesignCraft\designcraft.exe"
Remove-Item Env:WGPU_BACKEND
```

The override is local to that PowerShell session. Remove it after the diagnostic run so the
next launch uses normal backend selection. Keep the startup log when reporting a driver issue.

For the same hardware on Linux, see the [Ubuntu graphics guide](ubuntu.md#graphics-hardware).
