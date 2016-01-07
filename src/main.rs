extern crate byteorder;
extern crate time;

use std::path::Path;
use std::io::Cursor;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::string::String;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::OpenOptions;
use std::env;

struct SknVertex {
	position: [f32; 3],
	boneIndices:  [u8; 4],
	boneWeights:  [f32; 4],
	normal:  [f32; 3],
	u:  f32,
	v:  f32
}


fn cover4(buffer: [u8;4]) -> u32 {
        let magic: u32 = (buffer[0] as u32)  | (buffer[1] as u32) << 8 | (buffer[2] as u32) << 16 |(buffer[3] as u32) << 24;
        return magic;
}

fn cover2(buffer: [u8;2]) -> u32 {
        let magic: u32 = (buffer[0] as u32)  | (buffer[1] as u32) << 8;
        return magic;
}

fn main() {
	let pathstr = env::args().nth(1).unwrap();
	let path = Path::new(&pathstr);
	let file_name = path.file_name().unwrap().to_str().unwrap();
	let parent_dir = path.parent().unwrap().to_str().unwrap();

	let mut out_file = String::new();
	out_file.push_str(parent_dir);
	out_file.push('/');
	out_file.push_str(file_name);
	out_file.push_str(".dae");

	println!("{:?}", out_file);

    let mut f = File::open(&pathstr).unwrap();
    let mut buffer: [u8;4] = [0;4];
    // read the whole file
    f.read(&mut buffer);
	let mut rdr1 = Cursor::new(Vec::from(&buffer[..]));
    let magic: u32 = rdr1.read_u32::<LittleEndian>().unwrap();
    print!("magic: {}\n", magic);

    let mut buffer1 = [0;2];
    f.read(&mut buffer1);
	rdr1 = Cursor::new(Vec::from(&buffer1[..]));
	let fileVersion = rdr1.read_u16::<LittleEndian>().unwrap();
    //let fileVersion = cover2(buffer1);
    print!("fileVersion: {}\n", fileVersion);
    let mut matHeaderbuff = [0;2];
    f.read(&mut matHeaderbuff);
	rdr1 = Cursor::new(Vec::from(&matHeaderbuff[..]));
    let matHeader = rdr1.read_u16::<LittleEndian>().unwrap();

    if(fileVersion > 0){
        let mut numbermatbuff = [0;4];
        f.read(&mut numbermatbuff);
		rdr1 = Cursor::new(Vec::from(&numbermatbuff[..]));
        let numbermat = rdr1.read_u32::<LittleEndian>().unwrap();
        for i in 0..numbermat{
            f.seek(SeekFrom::Current(80));
        }
    }
    if(fileVersion == 4){
        f.seek(SeekFrom::Current(4));
    }

    let mut numIndicesbuff = [0; 4];
    let mut numVerticesbuff = [0; 4];
    f.read(&mut numIndicesbuff);
    f.read(&mut numVerticesbuff);

    let numIndices = cover4(numIndicesbuff);
    let numVertices = cover4(numVerticesbuff);

    if(fileVersion == 4){
        f.seek(SeekFrom::Current(48));
    }

    let mut indices: Vec<u32> = Vec::with_capacity(numIndices as usize);
    for i in 0..numIndices{
        let mut tmpbuff = [0;2];
        f.read(&mut tmpbuff);
        indices.push( cover2(tmpbuff));
    }

    let mut vertices: Vec<SknVertex> = Vec::with_capacity(numVertices as usize);
    let mut num = 0;

    for i in 0..numVertices{
        let mut newvert: SknVertex = SknVertex{
            position: [0f32,0f32,0f32],
            boneIndices: [0u8,0u8,0u8,0u8],
            boneWeights:[0f32,0f32,0f32,0f32],
            normal:[0f32,0f32,0f32],
            u: 0f32,
            v: 0f32,
        };
        vertices.push(newvert);

        let mut vert = &mut vertices[i as usize];
        let mut buff = [0;12];
        num = f.read(&mut buff).unwrap();
        if(num != 12){
            print!("zero");
            return;
        }
        let mut rdr = Cursor::new(Vec::from(&buff[0..4]));
        vert.position[0] = rdr.read_f32::<LittleEndian>().unwrap();
        rdr = Cursor::new(Vec::from(&buff[4..8]));
        vert.position[1] = rdr.read_f32::<LittleEndian>().unwrap();
        rdr = Cursor::new(Vec::from(&buff[8..12]));
        vert.position[2] = rdr.read_f32::<LittleEndian>().unwrap();


        let mut buff1 = [0;4];
        num = f.read(&mut buff1).unwrap();
        if(num != 4){
            print!("zero");
            return;
        }
        vert.boneIndices[0] = buff1[0];
        vert.boneIndices[1] = buff1[0];
        vert.boneIndices[2] = buff1[0];
        vert.boneIndices[3] = buff1[0];

        let mut buff2= [0;16];
        num = f.read(&mut buff2).unwrap();
        if(num != 16){
            print!("zero");
            return;
        }
        rdr = Cursor::new(Vec::from(&buff2[0..4]));
        vert.boneWeights[0] = rdr.read_f32::<LittleEndian>().unwrap();
        rdr = Cursor::new(Vec::from(&buff2[4..8]));
        vert.boneWeights[1] = rdr.read_f32::<LittleEndian>().unwrap();
        rdr = Cursor::new(Vec::from(&buff2[8..12]));
        vert.boneWeights[2] = rdr.read_f32::<LittleEndian>().unwrap();
        rdr = Cursor::new(Vec::from(&buff2[12..16]));
        vert.boneWeights[3] = rdr.read_f32::<LittleEndian>().unwrap();

        let mut buff3 = [0;12];
        num = f.read(&mut buff3).unwrap();
        if(num != 12){
            print!("zero");
            return;
        }

        rdr = Cursor::new(Vec::from(&buff3[0..4]));
        vert.normal[0] = rdr.read_f32::<LittleEndian>().unwrap();
        rdr = Cursor::new(Vec::from(&buff3[4..8]));
        vert.normal[1] = rdr.read_f32::<LittleEndian>().unwrap();
        rdr = Cursor::new(Vec::from(&buff3[8..12]));
        vert.normal[2] = rdr.read_f32::<LittleEndian>().unwrap();



        let mut buff4 = [0;4];
        num = f.read(&mut buff4).unwrap();
        if(num != 4){
            print!("zero");
            return;
        }
        rdr = Cursor::new(Vec::from(&buff4[0..4]));
        vert.u = rdr.read_f32::<LittleEndian>().unwrap();

        let mut buff5 = [0;4];
        num = f.read(&mut buff5).unwrap();
        if(num != 4){
            print!("zero");
            return;
        }
        rdr = Cursor::new(Vec::from(&buff5[0..4]));
        vert.v = 1f32 - rdr.read_f32::<LittleEndian>().unwrap();
    }
    setweight(&mut vertices);
    output(&out_file, &mut vertices, &mut indices);
}



fn setweight(vertices: &mut Vec<SknVertex>){
    for mut v in vertices{
        let totalWeight: f32 = v.boneWeights[0] + v.boneWeights[1] + v.boneWeights[2] + v.boneWeights[3];
        let weightError: f32 = 1f32 - totalWeight;
        if(weightError != 0f32){
            for j in 0..4{
                if(v.boneWeights[j] > 0f32){
                    v.boneWeights[j] += v.boneWeights[j] / totalWeight * weightError;
                }
            }
        }
    }
}

fn output(path: &String, vertices: &mut Vec<SknVertex>, indices: &mut Vec<u32>) {
    //let mut f = OpenOptions::new().read(true).write(true).open("akali.dae").unwrap();
	let mut f  = File::create(path).unwrap();
	f.write_all(b"<?xml version=\"1.0\" encoding=\"utf-8\"?>");
    f.write_all(b"<COLLADA xmlns=\"http://www.collada.org/2005/11/COLLADASchema\" version=\"1.4.1\">");
	f.write_all(b"<asset>");
	f.write_all(b"<contributor>");
	f.write_all(b"<authoring_tool>lightlfyan@gmail.com</authoring_tool>");
	f.write_all(b"</contributor>");

	let now = time::get_time();
	let utc = time::at_utc(now);
	let local = time::at(now);
	let timestr = utc.rfc3339().to_string();
	print!("{:?}", timestr);
	let timebin = &timestr.into_bytes();
	f.write_all(b"<created>");
	f.write_all(timebin);
	f.write_all(b"</created>");
	f.write_all(b"<modified>");
	f.write_all(timebin);
	f.write_all(b"</modified>");
	// f.write_all(b"<created>2016-01-04T11:37:31Z</created>");
	// f.write_all(b"<modified>2016-01-04T11:37:31Z</modified>");
	f.write_all(b"<unit meter=\"0.01\" name=\"centimeter\"/>");
	f.write_all(b"<up_axis>Y_UP</up_axis>");
	f.write_all(b"</asset>");
	f.write_all(b"<library_effects>");
	f.write_all(b"<effect id=\"DefaultLambert\">");
	f.write_all(b"<profile_COMMON>");
	f.write_all(b"<technique sid=\"Base\">");
	f.write_all(b"<lambert>");
	f.write_all(b"<emission>");
	f.write_all(b"<color>0.0 0.0 0.0 1.0</color>");
	f.write_all(b"</emission>");
	f.write_all(b"<ambient>");
	f.write_all(b"<color>0.0 0.0 0.0 1.0</color>");
	f.write_all(b"</ambient>");
	f.write_all(b"<diffuse>");
	f.write_all(b"<color>0.5 0.5 0.5 1.0</color>");
	f.write_all(b"</diffuse>");
	f.write_all(b"</lambert>");
	f.write_all(b"</technique>");
	f.write_all(b"</profile_COMMON>");
	f.write_all(b"</effect>");
	f.write_all(b"</library_effects>");
	f.write_all(b"<library_materials>");
	f.write_all(b"<material id=\"DefaultMaterial\">");
	f.write_all(b"<instance_effect url=\"#DefaultLambert\"/>");
	f.write_all(b"</material>");
	f.write_all(b"</library_materials>");

    let mut positionString = String::new();
    let mut normalString = String::new();
    let mut textureString = String::new();
    let mut indiceString = String::new();
    let len  = vertices.len();

    for i in vertices{
        positionString.push_str(&i.position[0].to_string());
        positionString.push(' ');
        positionString.push_str(&i.position[1].to_string());
        positionString.push(' ');
        positionString.push_str(&i.position[2].to_string());
        positionString.push(' ');

        normalString.push_str(&i.normal[0].to_string());
        normalString.push(' ');
        normalString.push_str(&i.normal[1].to_string());
        normalString.push(' ');
        normalString.push_str(&i.normal[2].to_string());
        normalString.push(' ');

        textureString.push_str(&i.u.to_string());
        textureString.push(' ');
        textureString.push_str(&i.v.to_string());
        textureString.push(' ');

    }

    let len2 = indices.len();
    for i in indices{
        indiceString.push_str(&i.to_string());
        indiceString.push(' ');
    }
    f.write_all(b"<library_geometries>");
    f.write_all(b"<geometry id=\"Mesh\" name=\"Mesh\">");
    f.write_all(b"<mesh>");
    f.write_all(b"<source id=\"MeshPosition\">");
    f.write_all(b"<float_array id=\"MeshPositionArray\" count=\"");
    f.write_all(&(len*3).to_string().into_bytes());
    f.write_all(b"\">");
    f.write_all(&positionString.into_bytes());
    f.write_all(b"</float_array>");
    f.write_all(b"<technique_common> ");
    f.write_all(b"<accessor source=\"#MeshPositionArray\" count=\"");
    f.write_all(&(len).to_string().into_bytes());
    f.write_all(b"\" stride=\"3\"> ");
    f.write_all(b"<param name=\"X\" type=\"float\"/> ");
    f.write_all(b"<param name=\"Y\" type=\"float\"/> ");
    f.write_all(b"<param name=\"Z\" type=\"float\"/> ");
    f.write_all(b"</accessor>");
    f.write_all(b"</technique_common>");
    f.write_all(b"</source> ");
    f.write_all(b"<source id=\"MeshNormal\"> ");
    f.write_all(b"<float_array id=\"MeshNormalArray\" count=\"");
    f.write_all(&(len*3).to_string().into_bytes());
    f.write_all(b"\">");
    f.write_all(&normalString.into_bytes());
    f.write_all(b"</float_array> ");
    f.write_all(b"<technique_common> ");
    f.write_all(b"<accessor source=\"#MeshNormalArray\" count=\"");
    f.write_all(&(len).to_string().into_bytes());
    f.write_all(b"\" stride=\"3\"> ");
    f.write_all(b"<param name=\"X\" type=\"float\"/> ");
    f.write_all(b"<param name=\"Y\" type=\"float\"/> ");
    f.write_all(b"<param name=\"Z\" type=\"float\"/> ");
    f.write_all(b"</accessor> ");
    f.write_all(b"</technique_common> ");
    f.write_all(b"</source> ");
    f.write_all(b"<source id=\"MeshTexture\"> ");
    f.write_all(b"<float_array id=\"MeshTextureArray\" count=\"");
    f.write_all(&(len*2).to_string().into_bytes());
    f.write_all(b"\">");
    f.write_all(&textureString.into_bytes());
    f.write_all(b"</float_array> ");
    f.write_all(b"<technique_common> ");
    f.write_all(b"<accessor source=\"#MeshTextureArray\" count=\"");
    f.write_all(&(len).to_string().into_bytes());
    f.write_all(b"\" stride=\"2\"> ");
    f.write_all(b"<param name=\"S\" type=\"float\"/> ");
    f.write_all(b"<param name=\"T\" type=\"float\"/> ");
    f.write_all(b"</accessor> ");
    f.write_all(b"</technique_common> ");
    f.write_all(b"</source> ");
    f.write_all(b"<vertices id=\"MeshVertices\"> ");
    f.write_all(b"<input semantic=\"POSITION\" source=\"#MeshPosition\"/> ");
    f.write_all(b"</vertices>");
    f.write_all(b"<triangles count=\"");
    f.write_all(&(len2 / 3).to_string().into_bytes());
    f.write_all(b"\" material=\"DefaultMaterial\">");
    f.write_all(b"<input semantic=\"VERTEX\" offset=\"0\" source=\"#MeshVertices\"/> ");
    f.write_all(b"<input semantic=\"NORMAL\" offset=\"0\" source=\"#MeshNormal\"/> ");
    f.write_all(b"<input semantic=\"TEXCOORD\" offset=\"0\" set=\"0\" source=\"#MeshTexture\"/> ");
    f.write_all(b"<p>");
    f.write_all(&indiceString.into_bytes());
    f.write_all(b"</p>");
    f.write_all(b"</triangles>");
    f.write_all(b"</mesh>");
    f.write_all(b"</geometry>");
    f.write_all(b"</library_geometries>");
	f.write_all(b"<library_visual_scenes>");
	f.write_all(b"<visual_scene id=\"DefaultScene\">");
	f.write_all(b"<node id=\"Model\" name=\"Model\">");
	f.write_all(b"<translate>0 0 0</translate>");
	f.write_all(b"<rotate>0 0 1 0</rotate>");
	f.write_all(b"<rotate>0 1 0 0</rotate>");
	f.write_all(b"<rotate>1 0 0 0</rotate>");
	f.write_all(b"<scale>1 1 1</scale>");
	f.write_all(b"<instance_geometry url=\"#Mesh\">");
	f.write_all(b"<bind_material>");
	f.write_all(b"<technique_common>");
	f.write_all(b"<instance_material symbol=\"DefaultMaterial\" target=\"#DefaultMaterial\"/>");
	f.write_all(b"</technique_common>");
	f.write_all(b"</bind_material>");
	f.write_all(b"</instance_geometry>");
	f.write_all(b"</node>");
	f.write_all(b"</visual_scene>");
	f.write_all(b"</library_visual_scenes>");
	f.write_all(b"<scene>");
	f.write_all(b"<instance_visual_scene url=\"#DefaultScene\"/>");
	f.write_all(b"</scene>");
    f.write_all(b"</COLLADA>");
}
