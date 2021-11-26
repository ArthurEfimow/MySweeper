#version 330 core

out vec4 Color;

in vec2 TexCoord;

uniform sampler2D ourTexture;

void main()
{
    vec4 FragColor = texture(ourTexture, TexCoord);
    if(FragColor.a < 0.5)
        discard;
    Color = FragColor;
}
