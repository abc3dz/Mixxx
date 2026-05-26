shader_type spatial;

uniform vec4 color1 : source_color = vec4(0.25, 0.25, 0.25, 1.0);
uniform vec4 color2 : source_color = vec4(0.35, 0.35, 0.35, 1.0);

// แนะนำให้ปรับค่าให้น้อยลงหน่อยครับ เพราะพิกัดโลกหน่วยจะเป็นเมตร (เช่น 1.0 คือ 1 ตารางเมตร)
uniform float tile_size = 1.0; 

// ดึงค่าตำแหน่งในโลก 3D มาใช้
varying vec3 world_position;

void vertex() {
    // คำนวณหาตำแหน่งของ Vertex นั้นๆ ในโลก 3D แล้วส่งไปให้ Fragment
    world_position = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
}

void fragment() {
    // ใช้เทคนิค Triplanar-like approach แบบง่ายๆ 
    // หรือเลือกจับคู่แกนตามที่คุณต้องการ ในที่นี้ใช้ X และ Z สำหรับพื้น/เพดาน 
    // แต่ถ้าอยากให้ครอบคลุมกำแพงด้วย เราจะใช้พิกัด X, Y, Z มาผสมกัน
    
    // คำนวณตารางโดยอิงจากพิกัด X, Y, Z ในโลกจริง
    vec3 grid = floor(world_position / tile_size);

    // รวมค่าทั้ง 3 แกนเพื่อหาช่องสลับหมากรุก 3 มิติ
    float checker = mod(grid.x + grid.y + grid.z, 2.0);

    vec3 color = mix(color1.rgb, color2.rgb, checker);

    ALBEDO = color;
}