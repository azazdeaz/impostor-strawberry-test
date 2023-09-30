using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Obi;
using System.Collections.Generic;

public static class RangeHelper
{
    public static int[] Range(int start, int steps, int step)
    {
        List<int> rangeList = new List<int>();
        for (int i = 0; i < steps; i += 1)
        {
            rangeList.Add(start + i * step);
        }

        return rangeList.ToArray();
    }
}

[RequireComponent(typeof(MeshFilter), typeof(MeshRenderer))]
[RequireComponent(typeof(ObiCloth), typeof(ObiClothRenderer))]
[RequireComponent(typeof(ObiStitcher))]
public class Leaf : MonoBehaviour
{
    public int xResolution = 10;
    public int yResolution = 10;
        public Material rodMaterial;
    public Texture leafTexture;

    private void Start()
    {
        GenerateMesh();
    }

    private void GenerateMesh()
    {
        Mesh mesh = new Mesh();

        Vector3[] vertices = new Vector3[(xResolution + 1) * (yResolution + 1)];
        Vector2[] uv = new Vector2[vertices.Length];
        int[] triangles = new int[xResolution * yResolution * 6];

        int vert = 0;
        int tris = 0;

        for (int y = 0; y <= yResolution; y++)
        {
            for (int x = 0; x <= xResolution; x++)
            {
                vertices[vert] = new Vector3(x, 0, y);
                uv[vert] = new Vector2(x / (float)xResolution, y / (float)yResolution);

                if (x != xResolution && y != yResolution)
                {
                    triangles[tris + 0] = vert;
                    triangles[tris + 1] = vert + xResolution + 1;
                    triangles[tris + 2] = vert + 1;
                    triangles[tris + 3] = vert + 1;
                    triangles[tris + 4] = vert + xResolution + 1;
                    triangles[tris + 5] = vert + xResolution + 2;

                    tris += 6;
                }

                vert++;
            }
        }

        mesh.vertices = vertices;
        mesh.triangles = triangles;
        mesh.uv = uv;
        mesh.RecalculateNormals();

        GetComponent<MeshFilter>().mesh = mesh;

        // create the blueprint: (ObiClothBlueprint, ObiTearableClothBlueprint, ObiSkinnedClothBlueprint)
        var clothBlueprint = ScriptableObject.CreateInstance<ObiClothBlueprint>();

        // set the input mesh:
        clothBlueprint.inputMesh = mesh;

        // generate the clothBlueprint:
        // StartCoroutine(clothBlueprint.Generate());
        clothBlueprint.GenerateImmediate();

        // create the cloth actor/renderer:
        // GameObject clothObject = new GameObject("cloth", typeof(ObiCloth),typeof(ObiClothRenderer));
        ObiCloth cloth = gameObject.GetComponent<ObiCloth>();
        Renderer clothRenderer = gameObject.GetComponent<Renderer>();
        // set material for the leaf
        clothRenderer.material = new Material(Shader.Find("Unlit/Texture"));
        clothRenderer.material.mainTexture = leafTexture;

        // instantiate and set the clothBlueprint:
        cloth.clothBlueprint = ScriptableObject.Instantiate(clothBlueprint);

        // parent the cloth under a solver to start simulation:
        cloth.transform.parent = gameObject.transform;

        print("> cloth particles" + string.Join(", ", cloth.solverIndices));


        //// create a rope:

        // create the blueprint: (ltObiRopeBlueprint, ObiRodBlueprint)
        var ropeBlueprint = ScriptableObject.CreateInstance<ObiRodBlueprint>();

        // Procedurally generate the rope path (a simple straight line):
        int filter = ObiUtils.MakeFilter(ObiUtils.CollideWithEverything, 0);
        ropeBlueprint.path.Clear();
        System.Action<Vector3, string> addControlPoint = (position, name) =>
        {
            ropeBlueprint.path.AddControlPoint(position, -Vector3.right, Vector3.right, Vector3.up, 0.1f, 0.1f, 1,
                filter, Color.white, name);
        };

        addControlPoint(vertices[0], "start");
        addControlPoint(vertices[xResolution], "end");

        // generate the particle representation of the rope (wait until it has finished):
        ropeBlueprint.GenerateImmediate();


        // create bones:
        System.Func<int[], GameObject, GameObject[]> addAxe = (clothParticleIndices, parent) =>
        {
            GameObject[] axes = new GameObject[clothParticleIndices.Length];
            // iterate over transforms
            for (int i = 0; i < clothParticleIndices.Length; ++i)
            {
                print("Add bone at " + clothParticleIndices[i]);
                var position = clothBlueprint.positions[clothParticleIndices[i]];
                // create a gameobject for each transform:
                axes[i] = new GameObject("Axe_" + i);
                axes[i].transform.parent = parent.transform;
                axes[i].transform.position = position;

                parent = axes[i];
            }

            return axes;
        };


        var mainAxe = addAxe(RangeHelper.Range(0, xResolution + 1, 1), gameObject);
        var secondaryStartsAt = (xResolution + 1) / 2;
        var secondaryAxe = addAxe(RangeHelper.Range(secondaryStartsAt + xResolution+1, yResolution-1, xResolution+1),
            mainAxe[secondaryStartsAt]);

        ObiBone bone = mainAxe[0].AddComponent<ObiBone>();
        ObiParticleRenderer boneParticleRenderer = mainAxe[0].AddComponent<ObiParticleRenderer>();
        boneParticleRenderer.radiusScale = 2.0f;
        boneParticleRenderer.particleColor = Color.green;
        ObiBoneBlueprint boneBlueprint = bone.boneBlueprint;
        
        print("> bone particles" + string.Join(", ", boneBlueprint.activeParticleCount));
        // log bone particle positions
        for (int i = 0; i < boneBlueprint.activeParticleCount; ++i)
        {
            print("Bone particle " + i + " at " + boneBlueprint.positions[i]);
        }        
        // stitch the cloth to the rope:
        ObiStitcher stitcher = GetComponent<ObiStitcher>();
        stitcher.Actor1 = cloth;
        stitcher.Actor2 = bone;
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
            print("Stitch " + i + " to " + closestParticle + " at " + closestDistance + " distance");
            stitcher.AddStitch(closestParticle, i);
        }

        stitcher.PushDataToSolver();
        //
        // // create a rope:
        // GameObject ropeObject = new GameObject("rope", typeof(ObiRod), typeof(ObiRopeExtrudedRenderer));
        //
        // // get component references:
        // ObiRod rope = ropeObject.GetComponent<ObiRod>();
        // ObiRopeExtrudedRenderer ropeRenderer = ropeObject.GetComponent<ObiRopeExtrudedRenderer>();
        // MeshRenderer ropeMeshRenderer = ropeObject.GetComponent<MeshRenderer>();
        //
            // ropeMeshRenderer.material = rodMaterial;
        // ropeRenderer.uvScale.y = 10;
        //
        // // load the default rope section:
        // ropeRenderer.section = Resources.Load<ObiRopeSection>("DefaultRopeSection");
        // // instantiate and set the ropeBlueprint:
        // rope.rodBlueprint = ScriptableObject.Instantiate(ropeBlueprint);
        //
        // // parent the cloth under a solver to start simulation:
        // rope.transform.parent = gameObject.transform;
        // // get parent solver:
        // // ObiSolver solver = gameObject.GetComponentInParent<ObiSolver>();
        // // solver.AddActor(rope);
        //
        //
        // print("> rope particles" + string.Join(", ", rope.solverIndices));
        //
        //
        // // // stitch the cloth to the rope:
        // // ObiStitcher stitcher = GetComponent<ObiStitcher>();
        // // stitcher.Actor1 = cloth;
        // // stitcher.Actor2 = rope;
        // // // stitch each particle in the first row of the cloth to the closest particle in the rope:
        // // for (int i = 0; i <= xResolution; ++i)
        // // {
        // //     var position = clothBlueprint.positions[i];
        // //     // find the closest particle in the rope:
        // //     int closestParticle = 0;
        // //     float closestDistance = float.MaxValue;
        // //     for (int j = 0; j < ropeBlueprint.activeParticleCount; ++j)
        // //     {
        // //         var distance = Vector3.Distance(position, ropeBlueprint.positions[j]);
        // //         if (distance < closestDistance)
        // //         {
        // //             closestParticle = j;
        // //             closestDistance = distance;
        // //         }
        // //     }
        // //
        // //     stitcher.AddStitch(i, closestParticle);
        // // }
        // //
        // // stitcher.PushDataToSolver();
        //
        //
        // // create a sphere game object to pin the rope to:
        // System.Func<Vector3, ObiCollider> addPinObject = (position) =>
        // {
        //     GameObject sphere = GameObject.CreatePrimitive(PrimitiveType.Sphere);
        //     sphere.transform.localScale = Vector3.one * 0.1f;
        //     sphere.transform.position = position;
        //     sphere.transform.parent = gameObject.transform;
        //     return sphere.AddComponent<ObiCollider>();
        // };
        // var pinObject1 = addPinObject(vertices[0]);
        // var pinObject2 = addPinObject(vertices[xResolution - 1]);
        //
        //
        // // get a hold of the constraint type we want, in this case, pin constraints:
        // var pinConstraints =
        //     rope.GetConstraintsByType(Oni.ConstraintType.Pin) as ObiConstraints<ObiPinConstraintsBatch>;
        //
        // // remove all batches from it, so we start clean:
        // pinConstraints.Clear();
        //
        // // create a new pin constraints batch
        // var batch = new ObiPinConstraintsBatch();
        //
        // // Add a couple constraints to it, pinning the first and last particles in the rope:
        // batch.AddConstraint(rope.solverIndices[0], pinObject1, Vector3.zero, Quaternion.identity, 0, 0,
        //     float.PositiveInfinity);
        // batch.AddConstraint(rope.solverIndices[ropeBlueprint.activeParticleCount - 1], pinObject2, Vector3.zero,
        //     Quaternion.identity, 0, 0, float.PositiveInfinity);
        //
        // // set the amount of active constraints in the batch to 2 (the ones we just added).
        // batch.activeConstraintCount = 2;
        //
        // // append the batch to the pin constraints:
        // pinConstraints.AddBatch(batch);
        //
        // // this will cause the solver to rebuild pin constraints at the beginning of the next frame:
        // rope.SetConstraintsDirty(Oni.ConstraintType.Pin);
    }
}