# Shader Template Exporter
## Project Description
***
The Shader Template Exporter is a collection of shader script snippets to be inserted into templates dynamically and exported to any given output directory. The exporter offers a Color Vision Deficiency Accessibility Shader as an example generic core and template.  

The shader template exporter is designed to make it easier to use basic shaders in most game engines.
## Project Objectives
***
#### Objective 1:
Build an exporter script in Python that exports data to multiple scripting languages.
##### Activities:
1. Write base language exporter class by implementing universal things
2. Derive language-specific child classes for using specific variable types (float3x3, float4, etc...)
3. Export the script based on user input from the command line
#### Objective 2:
Find the proper colorspace conversion and color-shifting algorithms and matrices.
##### Activities:
1. Read color vision deficiency research papers to determine the best color-shifting matrices for each CVD type.
2. Test those matrices in Unity, and then all supported engines.
#### Objective 3:
Implement basic interfaces for game engines with the pre-generated scripts.
##### Activities:
1. Write a Unity interface with the imported HLSL shader code using the shader and material file types
2. Write an Unreal Engine interface with the imported HLSL shader code

## Implementation of the Shader Exporting
***
Shader exporting is similar to how static website generators work, take a template and operate on it depending on the context. By having a tag in the template file – something like `// <% core_here %>` – the exporter can place the core into the template where the user specifies.

The shader exporter will have a singular 

Interface example:
```C
/* template/Unity.hlsl */

// <% core_here %>

void main() {
	/* Shader code here! */
}
```

Core example:
```c
/* cvd/core.hlsl */

float3 RGB_to_LMS(float3 rgb) ...
float3 LMS_to_RGB(float3 lms) ...

float3 Daltonize(float3 lms, int type) ...
```

Result from the generation :
```C
/* gen/Unity_gen.hlsl */

float3 RGB_to_LMS(float3 rgb) ...
float3 LMS_to_RGB(float3 lms) ...

float3 Daltonize(float3 lms, int type) ...

void main() {
	/* Shader code here! */
}
```