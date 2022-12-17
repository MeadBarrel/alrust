use crate::mask::mut_mask;

pub type Features = Vec<f64>;

pub fn paretto_assess(src: &[Features]) -> Vec<(u64, f64)> {
    let candidates: Vec<Features> = src.to_vec();

    let mut ranks = vec![0; candidates.len()];
    let mut mask = vec![true; candidates.len()];
    let mut rank: u64 = 0;

    while mask.iter().any(|x| *x) {
        let mut efficiency_mask = mask.clone();

        get_paretto_efficient_mask_masked(&candidates, &mut efficiency_mask);

        for (i, &flag) in efficiency_mask.iter().enumerate() {
            if flag {
                ranks[i] = rank;
            }
        }

        rank += 1;
        //invert_mask(&mut mask);
        for i in 0..efficiency_mask.len() {
            if efficiency_mask[i] {
                mask[i] = false;
            }
        }
    }

    let crowding_distances = crowding_distance(&candidates);

    let mut result = Vec::with_capacity(candidates.len());

    for i in 0..crowding_distances.len() {
        result.push((ranks[i], crowding_distances[i]))
    }

    result
}

pub fn get_paretto_efficient_mask<'a>(src: &[Features]) -> Vec<bool> {
    let mut non_dominated_points_mask = vec![true; src.len()];

    get_paretto_efficient_mask_masked(src, &mut non_dominated_points_mask);

    non_dominated_points_mask
}

pub fn get_paretto_efficient_mask_masked(src: &[Features], mask: &mut Vec<bool>) {
    for (i, features) in src.iter().enumerate() {
        if !mask[i] {
            continue;
        }
        mut_mask(src, mask, |x| {
            x.iter().zip(features.iter()).any(|(&x, &o)| o < x)
        });
        mask[i] = true;
    }
}

pub fn crowding_distance(src: &Vec<Features>) -> Vec<f64> {
    if src.len() == 0 {
        return Vec::new();
    }
    let num_features = src[0].len();
    let inf = f64::INFINITY;
    let mut distances = vec![0.; src.len()];
    let last_distance = distances.len() - 1;
    let mut src_temp: Vec<(usize, &Features)> = src.iter().enumerate().collect();

    for i in 0..num_features {
        src_temp.sort_by_key(|(_, x)| (x[i] * 1e+10) as u64);

        distances[src_temp[0].0] = inf;
        distances[src_temp[last_distance].0] = inf;
        let norm = src_temp[last_distance].1[i] - src_temp[0].1[i];

        if norm == 0. {
            continue;
        }

        for j in 1..last_distance {
            let difference = src_temp[j + 1].1[i] - src_temp[j - 1].1[i];
            distances[src_temp[j].0] += difference / norm;
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mask::indices_from_mask;
    use float_cmp::approx_eq;

    #[test]
    fn test_assess() {
        let points = vec![
            vec![0.7873616773923351, 0.8092306552000161],
            vec![0.21173326011754878, 0.6339482992398732],
            vec![0.01725675132713511, 0.9881718237619325],
            vec![0.5330575947747812, 0.9857357852889478],
            vec![0.5829186417619112, 0.5495024479309618],
            vec![0.3521920654953825, 0.9142557605053708],
            vec![0.3692810621902112, 0.08987228791660551],
            vec![0.7478009420313325, 0.3523304812577952],
            vec![0.5212182747402428, 0.41024277235906326],
            vec![0.6000844913877189, 0.3594561767427774],
            vec![0.21823414269097896, 0.8820946957442006],
            vec![0.4550299655344954, 0.6162078310693472],
            vec![0.17710113749892753, 0.006050443424864049],
            vec![0.744808216764824, 0.11893987805784223],
            vec![0.08517607238714664, 0.5755688995187869],
            vec![0.0311175093718834, 0.14680352435542987],
            vec![0.9406975842111823, 0.36328027015743847],
            vec![0.49042703806432253, 0.21626967830636024],
            vec![0.11508201721525246, 0.9030739711618478],
            vec![0.7212068364234988, 0.1843117185801686],
            vec![0.4720653136444348, 0.32004342948828035],
            vec![0.7285032162065087, 0.38694809427377175],
            vec![0.47187397860969094, 0.7384091109267817],
            vec![0.17896648902209567, 0.779927706301946],
            vec![0.2441719609991977, 0.8338028399022548],
            vec![0.6138092170178797, 0.096676922062096],
            vec![0.03926807017744294, 0.6405796332697564],
            vec![0.3597415915757063, 0.7480627116447116],
            vec![0.8332102156679112, 0.23308833651764094],
            vec![0.7571160739197182, 0.9997153582193037],
        ];

        let inf = f64::INFINITY;

        let expected = vec![
            (0, 0.13662144722870032),
            (4, 0.06705037798426683),
            (1, inf),
            (1, 0.14120303635919387),
            (1, 0.23896401583642363),
            (2, 0.2083399165100112),
            (4, 0.1943926566786755),
            (1, 0.05299227965872994),
            (2, 0.20975561802766074),
            (2, 0.04447120104564087),
            (3, 0.10484084479780645),
            (3, 0.16985012321216233),
            (5, inf),
            (2, 0.07134381421489303),
            (5, 0.14923007104520658),
            (6, 0.08962481535931327),
            (0, inf),
            (3, 0.10231564880634747),
            (3, 0.13191234933490628),
            (2, 0.1941119008950743),
            (3, 0.1400935970439103),
            (1, 0.07281999655957663),
            (2, 0.12661602170405725),
            (4, 0.09906126589340619),
            (3, 0.2183924770512246),
            (3, 0.16041707402003105),
            (5, 0.16366716348478308),
            (2, 0.060289080229606086),
            (1, 0.2704837949183645),
            (0, inf),
        ];

        let actual = paretto_assess(&points);

        for (a, e) in actual.into_iter().zip(expected.into_iter()) {
            assert!(approx_eq!(f64, a.1, e.1, epsilon = 0.001) && a.0 == e.0)
        }
    }

    #[test]
    fn test_crowding_distance() {
        let points = vec![
            vec![0.7, 0.1],
            vec![0.5, 0.8],
            vec![0.1, 0.7],
            vec![0.6, 0.5],
            vec![0.3, 0.9],
        ];

        let inf = f64::INFINITY;

        let expected = vec![inf, 0.75, inf, 1.083, inf];

        let actual = crowding_distance(&points);

        for (&a, &e) in actual.iter().zip(expected.iter()) {
            assert!(approx_eq!(f64, a, e, epsilon = 0.001), "{} != {}", a, e)
        }
    }

    #[test]
    fn test_paretto_front() {
        let points = vec![
            vec![0.7139324067489564, 0.21330768149414225],
            vec![0.9632557239320335, 0.8167846315250179],
            vec![0.6573584326863099, 0.4071925634090626],
            vec![0.5754982241840986, 0.9783284777821567],
            vec![0.5941226989306109, 0.24393665217713867],
            vec![0.5678146754392837, 0.433138558343813],
            vec![0.927064267041476, 0.8355616370730305],
            vec![0.09859252646890593, 0.9060615129831883],
            vec![0.45572879803040367, 0.38317673908009864],
            vec![0.3171025340316248, 0.3504734383622189],
            vec![0.1259074622769889, 0.28808108305489954],
            vec![0.22557357439411696, 0.12481113773499053],
            vec![0.3817950615513077, 0.7907097996386792],
            vec![0.6515235310851071, 0.4946080623443133],
            vec![0.7054542504808121, 0.6794859028349699],
            vec![0.3005253845771516, 0.08818143205249818],
            vec![0.4110841005711444, 0.9072445623403698],
            vec![0.028545632254259323, 0.5032475157985616],
            vec![0.6107229215275294, 0.6429565192890805],
            vec![0.1035356291668944, 0.33967655670642194],
            vec![0.012004446834854687, 0.9938649671958041],
            vec![0.4547392241676276, 0.5021329994449573],
            vec![0.6599594847241971, 0.0963292553889532],
            vec![0.5197847974415288, 0.7864365085775598],
            vec![0.020111774836352736, 0.3690756863700899],
            vec![0.4164300879729963, 0.5644955038343611],
            vec![0.33361111115919584, 0.7214136853555878],
            vec![0.7200409206553287, 0.43783989791183153],
            vec![0.389080780478117, 0.6837673677427005],
            vec![0.34373092905463143, 0.6413793401622063],
            vec![0.3740728336815984, 0.571324726301669],
            vec![0.6385956641532615, 0.8775555758478206],
            vec![0.6620219997615562, 0.9198115344285939],
            vec![0.7817375222291649, 0.8599120218114593],
            vec![0.8985371829203493, 0.18481777508521158],
            vec![0.1292146226515345, 0.9468167673655171],
            vec![0.3053318079253662, 0.593106037444356],
            vec![0.8270438088566954, 0.014335633381665769],
            vec![0.9693648303302085, 0.4295233511438866],
            vec![0.8945495252806903, 0.23354853505494244],
            vec![0.6684186673810997, 0.6699736145902835],
            vec![0.37855043084731654, 0.372892021974477],
            vec![0.9305575480845234, 0.552741362558818],
            vec![0.45448544271449176, 0.7056894402687941],
            vec![0.3769084384086412, 0.9989010785716839],
            vec![0.549160360174713, 0.8457882716156453],
            vec![0.2040130665742247, 0.8805691441193322],
            vec![0.05975702586186671, 0.3698804786411396],
            vec![0.7447345021779552, 0.7129930006020961],
            vec![0.624082199919555, 0.44298877462335495],
        ];
        let mut expected: Vec<usize> = vec![1, 6, 33, 32, 3, 38, 44];

        expected.sort();

        let mask = get_paretto_efficient_mask(&points);
        let mut indices: Vec<usize> = indices_from_mask(&mask);

        indices.sort();

        assert!(expected == indices, "{:?}", indices)
    }
}
