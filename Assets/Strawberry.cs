using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using Obi;
using Unity.VisualScripting;

public class Bone
{
    public uint start;
    public uint end;

    public Bone(uint start, uint end)
    {
        this.start = start;
        this.end = end;
    }
}

public class Blade
{
    public uint[] triangles;
}

public class PlantDescription
{
    public Vector3[] vertices;
    public Bone[] bones;
    public Blade[] blades;
}

// [RequireComponent(typeof(MeshRenderer))]
public class Strawberry : MonoBehaviour
{
    
    public Texture leafTexture;
    
    // Start is called before the first frame update
    void Start()
    {
        // create basic plant description:
        var plantDescription = new PlantDescription();
        var up = Vector3.up;
        var side = Vector3.right;
        var scale = 1.1f;

        plantDescription.vertices = new Vector3[]
        {
            up * 0.0f,
            up * 0.2f,
            up * 0.4f,
            up * 0.6f,
            up * 0.8f,
            up * 1.0f,
            // start blade
            up * 1.1f + side * 0.0f,
            up * 1.2f + side * 0.1f,
            up * 1.3f + side * 0.2f,
            up * 1.2f + side * -0.1f,
            up * 1.3f + side * 0.0f,
            up * 1.4f + side * 0.1f,
            up * 1.2f + side * -0.2f,
            up * 1.3f + side * -0.1f,
            up * 1.4f + side * 0.0f,
        };
        for (var i = 0; i < plantDescription.vertices.Length; i++)
        {
            plantDescription.vertices[i] *= scale;
        }
        plantDescription.bones = new Bone[]
        {
            new Bone(0, 1),
            new Bone(1, 2),
            new Bone(2, 3),
            new Bone(3, 4),
            new Bone(4, 5),
            new Bone(5, 6),
            // midrib
            new Bone(6, 10),
            new Bone(10, 14),
            // left blade
            new Bone(6, 7),
            new Bone(7, 8),
            // right blade
            new Bone(6, 9),
            new Bone(9, 12),
        };
        
        var first_blade = new Blade();
        first_blade.triangles = new uint[] { 6, 7, 8, 6, 8, 9, 6, 9, 12, 6, 12, 13, 6, 13, 14 };
        // first_blade.triangles = new uint[] { 5, 7, 9, 5, 11, 7 };
        plantDescription.blades = new Blade[] { first_blade };
        
        // Build bones
        var boneGameObject = new Dictionary<uint, GameObject>();
        var rootAxe = new GameObject("RootAxe");
        rootAxe.transform.parent = gameObject.transform;
        rootAxe.transform.position = Vector3.zero;
        boneGameObject[0] = rootAxe;
        
        foreach (Bone bone in plantDescription.bones) {
            var parent = boneGameObject[bone.start];
            // create a gameobject for each transform:
            var axe = new GameObject("Axe_" + bone.start + "_" + bone.end);
            axe.transform.parent = parent.transform;
            axe.transform.position = plantDescription.vertices[bone.end];// - plantDescription.vertices[bone.start];
            boneGameObject[bone.end] = axe;
        };
        
        ObiBone obiBone = rootAxe.AddComponent<ObiBone>();
        ObiParticleRenderer boneParticleRenderer = rootAxe.AddComponent<ObiParticleRenderer>();
        boneParticleRenderer.radiusScale = 0.3f;
        boneParticleRenderer.particleColor = Color.green;
        boneParticleRenderer.shader = Shader.Find("Obi/Particles");
        ObiBoneBlueprint boneBlueprint = obiBone.boneBlueprint;
        
        
        // Build meshes
        foreach (Blade blade in plantDescription.blades)
        {
            var mesh = new Mesh();
            // remove all the vertices that are not part of the blade and update indices
            var remapIndices = new List<uint>();
            foreach (uint index in blade.triangles)
            {
                if (!remapIndices.Contains(index))
                {
                    remapIndices.Add(index);
                }
            }
            var vertices = remapIndices.Select(index => plantDescription.vertices[index]).ToArray();
            var triangles = blade.triangles.Select(index => (int)remapIndices.IndexOf(index)).ToArray();
            
            
            mesh.vertices = vertices;
            mesh.triangles = triangles;
            mesh.RecalculateNormals();
            
            // Mesh mesh = new Mesh();
            // var xResolution = 6;
            // var yResolution = 6;
            //
            // Vector3[] vertices = new Vector3[(xResolution + 1) * (yResolution + 1)];
            // Vector2[] uv = new Vector2[vertices.Length];
            // int[] triangles = new int[xResolution * yResolution * 6];
            //
            // int vert = 0;
            // int tris = 0;
            //
            // for (int y = 0; y <= yResolution; y++)
            // {
            //     for (int x = 0; x <= xResolution; x++)
            //     {
            //         vertices[vert] = new Vector3(x, y, 0);
            //         uv[vert] = new Vector2(x / (float)xResolution, y / (float)yResolution);
            //
            //         if (x != xResolution && y != yResolution)
            //         {
            //             triangles[tris + 0] = vert;
            //             triangles[tris + 1] = vert + xResolution + 1;
            //             triangles[tris + 2] = vert + 1;
            //             triangles[tris + 3] = vert + 1;
            //             triangles[tris + 4] = vert + xResolution + 1;
            //             triangles[tris + 5] = vert + xResolution + 2;
            //
            //             tris += 6;
            //         }
            //
            //         vert++;
            //     }
            // }
            
            // // scale the vertices:
            // for (int i = 0; i < vertices.Length; i++)
            // {
            //     vertices[i] *= 1.0f / yResolution;
            // }
            
            // mesh.vertices = vertices;
            // mesh.triangles = triangles;
            // mesh.uv = uv;
            // mesh.RecalculateNormals();
            
            var bladeGameObject = new GameObject("Blade");
            bladeGameObject.transform.parent = transform;
            bladeGameObject.AddComponent<MeshFilter>().mesh = mesh;
            bladeGameObject.AddComponent<MeshRenderer>();
            bladeGameObject.AddComponent<ObiClothRenderer>();
            bladeGameObject.AddComponent<ObiCloth>();
            bladeGameObject.AddComponent<ObiStitcher>();
            var clothParticleRenderer = bladeGameObject.AddComponent<ObiParticleRenderer>();
            
            
            clothParticleRenderer.radiusScale = 0.3f;
            clothParticleRenderer.particleColor = Color.red;
            clothParticleRenderer.shader = Shader.Find("Obi/Particles");
            
            // create the blueprint: (ObiClothBlueprint, ObiTearableClothBlueprint, ObiSkinnedClothBlueprint)
            var clothBlueprint = ScriptableObject.CreateInstance<ObiClothBlueprint>();
            
            // set the input mesh:
            clothBlueprint.inputMesh = mesh;
            
            // generate the clothBlueprint:
            // StartCoroutine(clothBlueprint.Generate());
            clothBlueprint.GenerateImmediate();
            
            // create the cloth actor/renderer:
            ObiCloth obiCloth = bladeGameObject.GetComponent<ObiCloth>();
            Renderer clothRenderer = bladeGameObject.GetComponent<Renderer>();
            // set material for the leaf
            clothRenderer.material = new Material(Shader.Find("Unlit/Texture"));
            clothRenderer.material.mainTexture = leafTexture;
            
            // instantiate and set the clothBlueprint:  
            obiCloth.clothBlueprint = ScriptableObject.Instantiate(clothBlueprint);
            
            // // parent the cloth under a solver to start simulation:
            // obiCloth.transform.parent = bladeGameObject.transform;
            
            print("> cloth particles" + string.Join(", ", obiCloth.solverIndices));
            print(" > cloth blueprint particles" + string.Join(", ", clothBlueprint.positions));
            
            
            // Stitch the blade to the bone
            ObiStitcher stitcher = bladeGameObject.GetComponent<ObiStitcher>();
            stitcher.Actor1 = obiCloth;
            stitcher.Actor2 = obiBone;
            // stitch each particle in the first row of the cloth to the closest particle in the rope:
            for (int i = 0; i < boneBlueprint.activeParticleCount; ++i)
            {
                var position = boneBlueprint.positions[i];
                // find the closest particle in the cloth:
                int closestParticle = 0;
                float closestDistance = float.MaxValue;
                for (int j = 0; j < clothBlueprint.activeParticleCount; ++j)
                {
                    var distance = Vector3.Distance(position, clothBlueprint.positions[j]);
                    if (distance < closestDistance)
                    {
                        closestParticle = j;
                        closestDistance = distance;
                    }
                }
                if (closestDistance > 0.1f)
                {
                    print("Skip stitch " + i + " to " + closestParticle + " at " + closestDistance + " distance");
                    continue;
                }
                print("Stitch " + i + " to " + closestParticle + " at " + closestDistance + " distance");
                stitcher.AddStitch(closestParticle, i);
            }
            
            stitcher.PushDataToSolver();
        }
    }

    // Update is called once per frame
    void Update()
    {
    }
}