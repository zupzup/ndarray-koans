#[cfg(test)]
mod initialisation_input{
    use ndarray::{Array2, array, ArrayView1, Array, ArrayView2, stack, Axis, s};
    use rand_isaac::Isaac64Rng;
    use ndarray_rand::{RandomExt, rand};
    use ndarray_rand::rand::{SeedableRng, Rng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_npy::write_npy;

    // K-means, as the name says, requires you to declare upfront `k`: the number of clusters you are
    // looking to spot in your dataset (quite a strong assumption to make, I agree).
    //
    // When implementing the standard K-means algorithm, the most common initialisation
    // technique is the Forgy method: as your first set of centroids, just pick `n_clusters`
    // distinct observations from your dataset - as simple as that (and it works quite well!).
    pub fn get_random_centroids(n_clusters: usize, observations: __, rng: &mut impl Rng) -> Array2<f64> {
        __
    }

    // Helper function.
    // Check if there is at least one row in `matrix` that is equal to `row`
    fn is_row_of(matrix: &Array2<f64>, row: &ArrayView1<f64>) -> bool {
        matrix.genrows().into_iter().any(|r| &r == row)
    }

    #[test]
    fn input() {
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let n_observations = 50;
        let n_clusters = 3;
        let observations: Array2<f64> = Array::random_using((n_observations, n_clusters), StandardNormal, &mut rng);

        let centroids = get_random_centroids(n_clusters, observations.view(), &mut rng);

        // Centroids are a subset of our observations:
        // each one of them corresponds to a row in `observations`.
        assert!(centroids.genrows().into_iter().all(|centroid| is_row_of(&observations, &centroid)));
    }

    #[test]
    #[should_panic]
    // If the number of clusters we are looking for is bigger than the number of
    // available observations `get_random_centroids` should panic
    fn invalid_input() {
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let n_observations = 4;
        let n_clusters = 5;
        assert!(n_observations < n_clusters);
        let observations: Array2<f64> = Array::random_using((n_observations, n_clusters), StandardNormal, &mut rng);

        get_random_centroids(n_clusters, observations.view(), &mut rng);
    }
}
