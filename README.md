## Build Instructions

This document outlines the build process for different target platforms.

### Prerequisites

*   A C compiler (e.g., GCC)

### Build Commands

1.  **Compile the Builder:**

    ```bash
    gcc builder/build.c builder/functions.c -o ./build 
    ```

    *   **Note:**  The `-arch arm64` flag might be needed for cross-compilation on non-ARM64 systems.  Consider adding a conditional check in your build script or providing separate instructions.

    ```bash
    gcc -arch arm64 builder/build.c builder/functions.c -o ./build 
    ```

2.  **Build for Target Platform:**

    ```bash
    ./build <target>
    ```

    Where `<target>` can be one of the following:

    *   `applesilicon`:  For Apple Silicon (ARM64)
    *   `applex86`: For Apple Intel (x86)
    *   `linux`: For Linux

    **Example:** To build for Apple Silicon:

    ```bash
    ./build applesilicon
    ```

3.  **Clean Previous Build:**

    ```bash
    ./build clean
    ```

### Notes

*   Ensure the `build` executable has execute permissions (`chmod +x build`).
*   The `functions.c` file (mentioned in the original Apple Intel build command) should be included in the compilation step if it contains necessary functions.  If it's not needed, remove it to simplify the build process.