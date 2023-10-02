using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Obi;

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
    // Start is called before the first frame update
    void Start()
    {
        // create basic plant description:
        var plantDescription = new PlantDescription();
        var up = Vector3.up;
        var side = Vector3.right;

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
        
        var blade = new Blade();
        blade.triangles = new uint[] { 6, 7, 8, 6, 8, 9, 6, 9, 12, 6, 12, 13, 6, 13, 14 };
        plantDescription.blades = new Blade[] { blade };

        var boneGameObject = new Dictionary<uint, GameObject>();
        var rootAxe = new GameObject("RootAxe");
        rootAxe.transform.parent = gameObject.transform;
        rootAxe.transform.position = Vector3.zero;
        boneGameObject[0] = rootAxe;
        
        foreach (Bone bone in plantDescription.bones) {
            var parent = boneGameObject[bone.start];
            // create a gameobject for each transform:
            var axe = new GameObject("Axe_" + boneGameObject.Count);
            axe.transform.parent = parent.transform;
            axe.transform.position = plantDescription.vertices[bone.end];// - plantDescription.vertices[bone.start];
            boneGameObject[bone.end] = axe;
        };
        
        ObiBone obiBone = rootAxe.AddComponent<ObiBone>();
        ObiParticleRenderer boneParticleRenderer = rootAxe.AddComponent<ObiParticleRenderer>();
        boneParticleRenderer.radiusScale = 2.0f;
        boneParticleRenderer.particleColor = Color.green;
        ObiBoneBlueprint boneBlueprint = obiBone.boneBlueprint;
    }

    // Update is called once per frame
    void Update()
    {
    }
}