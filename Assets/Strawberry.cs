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
        boneParticleRenderer.radiusScale = 2.0f;
        boneParticleRenderer.particleColor = Color.green;
        ObiBoneBlueprint boneBlueprint = obiBone.boneBlueprint;
        
        
        // Build meshes
        foreach (Blade blade in plantDescription.blades)
        {
            var mesh = new Mesh();
            mesh.vertices = plantDescription.vertices;
            mesh.triangles = blade.triangles.Select(v => (int)v).ToArray();
            mesh.RecalculateNormals();
            
            var bladeGameObject = new GameObject("Blade");
            var bladeGameObjectMeshFilter = bladeGameObject.AddComponent<MeshFilter>();
            bladeGameObjectMeshFilter.mesh = mesh;
            bladeGameObjectMeshFilter.transform.parent = gameObject.transform;

            // create the blueprint: (ObiClothBlueprint, ObiTearableClothBlueprint, ObiSkinnedClothBlueprint)
            var clothBlueprint = ScriptableObject.CreateInstance<ObiClothBlueprint>();

            // set the input mesh:
            clothBlueprint.inputMesh = mesh;

            // generate the clothBlueprint:
            // StartCoroutine(clothBlueprint.Generate());
            clothBlueprint.GenerateImmediate();

            // create the cloth actor/renderer:
            // GameObject clothObject = new GameObject("cloth", typeof(ObiCloth),typeof(ObiClothRenderer));
            ObiCloth obiCloth = bladeGameObject.AddComponent<ObiCloth>();
            bladeGameObject.AddComponent<ObiClothRenderer>();
            bladeGameObject.AddComponent<MeshRenderer>();
            
            Renderer clothRenderer = gameObject.GetComponent<Renderer>();
            // set material for the leaf
            clothRenderer.material = new Material(Shader.Find("Unlit/Texture"));
            clothRenderer.material.mainTexture = leafTexture;

            // instantiate and set the clothBlueprint:
            obiCloth.clothBlueprint = ScriptableObject.Instantiate(clothBlueprint);

            // parent the cloth under a solver to start simulation:
            obiCloth.transform.parent = gameObject.transform;

            print("> cloth particles" + string.Join(", ", obiCloth.solverIndices));
            
            
            // Stitch the blade to the bone
            ObiStitcher stitcher = bladeGameObject.AddComponent<ObiStitcher>();
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
                    continue;
                }
                print("Stitch " + i + " to " + closestParticle + " at " + closestDistance + " distance");
                stitcher.AddStitch(closestParticle, i);
            }
        }
    }

    // Update is called once per frame
    void Update()
    {
    }
}