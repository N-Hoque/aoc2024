use std::collections::BTreeMap;

use itertools::Itertools;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileID(u64);

#[derive(Debug)]
pub struct DiskMap {
    disk_map: Vec<Option<FileID>>,
    file_map: BTreeMap<usize, u32>,
    sector_map: BTreeMap<usize, u32>,
}

impl DiskMap {
    pub fn parse(input: &str) -> Self {
        let (files, mut sectors): (Vec<_>, Vec<_>) = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .enumerate()
            .partition_map(|(idx, size)| {
                if idx % 2 == 0 {
                    itertools::Either::Left(size)
                } else {
                    itertools::Either::Right(size)
                }
            });

        if sectors.len() + 1 == files.len() {
            // Pad sectors with a trailing 0
            sectors.push(0);
        }

        let disk = files
            .iter()
            .enumerate()
            .zip(sectors.iter())
            .flat_map(|((file_id, file_size), sector_size)| {
                (0..*file_size)
                    .map(move |_| Some(FileID(file_id as u64)))
                    .chain((0..*sector_size).map(|_| None))
            })
            .collect::<Vec<_>>();

        Self {
            file_map: Self::file_map(&disk, &files),
            sector_map: Self::sector_map(&disk, &sectors),
            disk_map: disk,
        }
    }

    pub fn block_shrink(&self) -> Vec<Option<FileID>> {
        let mut data = self.disk_map.clone();

        // Swap the rightmode ID with the leftmost None until all None are at the end
        let mut left_index = 0;
        let mut right_index = data.len() - 1;

        loop {
            while data[left_index].is_some() {
                left_index += 1;
            }
            while data[right_index].is_none() {
                right_index -= 1;
            }

            if left_index >= right_index {
                break;
            }

            data.swap(left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }

        data
    }

    fn file_map(disk_map: &[Option<FileID>], files: &[u32]) -> BTreeMap<usize, u32> {
        let mut file_map = BTreeMap::new();
        let mut file_pointer = 0;
        let mut disk_pointer = 0;
        while disk_pointer < disk_map.len() {
            if disk_map[disk_pointer].is_some() {
                file_map.insert(disk_pointer, files[file_pointer]);
                disk_pointer += files[file_pointer] as usize;
                file_pointer += 1;
            } else {
                disk_pointer += 1;
            }
        }
        file_map
    }

    fn sector_map(disk_map: &[Option<FileID>], sectors: &[u32]) -> BTreeMap<usize, u32> {
        let mut sector_map = BTreeMap::new();
        let mut sector_pointer = 0;
        let mut disk_pointer = 0;
        while disk_pointer < disk_map.len() {
            if disk_map[disk_pointer].is_some() {
                disk_pointer += 1;
            } else {
                sector_map.insert(disk_pointer, sectors[sector_pointer]);
                disk_pointer += sectors[sector_pointer] as usize;
                sector_pointer += 1;
            }
        }
        sector_map
    }

    pub fn file_shrink(&self) -> Vec<Option<FileID>> {
        let mut data = self.disk_map.clone();

        let mut sector_map = self.sector_map.clone();

        for (file_id, file_size) in self.file_map.clone().iter().rev() {
            if let Some((sector_id, sector_size)) =
                sector_map.range(..file_id).find_map(|(sector_index, sector_size)| {
                    if *sector_size >= *file_size {
                        Some((*sector_index, *sector_size))
                    } else {
                        None
                    }
                })
            {
                // Move the file entries to the sector's position
                for i in 0..*file_size as usize {
                    data.swap(sector_id + i, file_id + i);
                }
                sector_map.remove(&sector_id);
                if (sector_size - *file_size) > 0 {
                    sector_map.insert(sector_id + *file_size as usize, sector_size - *file_size);
                }
            }
        }

        data
    }
}

fn checksum(disk_map: &[Option<FileID>]) -> u64 {
    disk_map.iter().enumerate().fold(0, |acc, (block_idx, file_id)| {
        acc + block_idx as u64 * file_id.map_or(0, |x| x.0)
    })
}

impl std::fmt::Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();

        for block in &self.disk_map {
            if let Some(file_id) = block {
                result += &format!("{}", file_id.0);
            } else {
                result.push('.');
            }
        }

        write!(f, "{}", result)
    }
}

pub fn solve_part_one(input: &str) -> u64 {
    let disk_map = DiskMap::parse(input);
    let disk_map = disk_map.block_shrink();
    let checksum = checksum(&disk_map);

    assert_eq!(checksum, 6471961544878); // Sanity check against accepted answer for part one
    checksum
}

pub fn solve_part_two(input: &str) -> u64 {
    let disk_map = DiskMap::parse(input);
    let disk_map = disk_map.file_shrink();
    let checksum = checksum(&disk_map);

    assert_eq!(checksum, 6511178035564); // Sanity check against accepted answer for part two
    checksum
}

#[cfg(test)]
mod test {
    use crate::FileID;

    static SAMPLE: &str = r"2333133121414131402";

    #[test]
    fn display_sample() {
        static TARGET: &str = "00...111...2...333.44.5555.6666.777.888899";
        let disk_map = super::DiskMap::parse(SAMPLE);
        assert_eq!(TARGET, disk_map.to_string());
    }

    #[test]
    fn block_shrink_sample() {
        let expected = vec![
            Some(FileID(0)),
            Some(FileID(0)),
            Some(FileID(9)),
            Some(FileID(9)),
            Some(FileID(8)),
            Some(FileID(1)),
            Some(FileID(1)),
            Some(FileID(1)),
            Some(FileID(8)),
            Some(FileID(8)),
            Some(FileID(8)),
            Some(FileID(2)),
            Some(FileID(7)),
            Some(FileID(7)),
            Some(FileID(7)),
            Some(FileID(3)),
            Some(FileID(3)),
            Some(FileID(3)),
            Some(FileID(6)),
            Some(FileID(4)),
            Some(FileID(4)),
            Some(FileID(6)),
            Some(FileID(5)),
            Some(FileID(5)),
            Some(FileID(5)),
            Some(FileID(5)),
            Some(FileID(6)),
            Some(FileID(6)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];

        let disk_map = super::DiskMap::parse(SAMPLE);

        let actual = disk_map.block_shrink();

        assert_eq!(expected, actual);
    }

    #[test]
    fn file_shrink_sample() {
        let expected = vec![
            Some(FileID(0)),
            Some(FileID(0)),
            Some(FileID(9)),
            Some(FileID(9)),
            Some(FileID(2)),
            Some(FileID(1)),
            Some(FileID(1)),
            Some(FileID(1)),
            Some(FileID(7)),
            Some(FileID(7)),
            Some(FileID(7)),
            None,
            Some(FileID(4)),
            Some(FileID(4)),
            None,
            Some(FileID(3)),
            Some(FileID(3)),
            Some(FileID(3)),
            None,
            None,
            None,
            None,
            Some(FileID(5)),
            Some(FileID(5)),
            Some(FileID(5)),
            Some(FileID(5)),
            None,
            Some(FileID(6)),
            Some(FileID(6)),
            Some(FileID(6)),
            Some(FileID(6)),
            None,
            None,
            None,
            None,
            None,
            Some(FileID(8)),
            Some(FileID(8)),
            Some(FileID(8)),
            Some(FileID(8)),
            None,
            None,
        ];

        let disk_map = super::DiskMap::parse(SAMPLE);

        let actual = disk_map.file_shrink();

        assert_eq!(expected, actual);
    }

    #[test]
    fn solve_sample_one() {
        let expected = 1928;

        let disk_map = super::DiskMap::parse(SAMPLE);
        let actual = disk_map
            .block_shrink()
            .iter()
            .enumerate()
            .fold(0, |acc, (block_idx, file_id)| {
                acc + if let Some(file_id) = file_id {
                    block_idx as u64 * file_id.0
                } else {
                    0
                }
            });

        assert_eq!(expected, actual);
    }

    #[test]
    fn solve_sample_two() {
        let expected = 2858;

        let disk_map = super::DiskMap::parse(SAMPLE);
        let actual = disk_map
            .file_shrink()
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, file_id)| {
                acc + if let Some(file_id) = file_id {
                    idx as u64 * file_id.0
                } else {
                    0
                }
            });

        assert_eq!(expected, actual);
    }
}
