/*
    Copyright (c) [2016] [ <maccoda> ]
    This file is part of the https://github.com/maccoda/diamond_square repository (8th of March 2018)
    file: diamond_square/src/diamond_square.rs
          
                                Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

    TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

    1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

    2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

    3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

    4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

    5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

    6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

    7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

    8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

    9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

    END OF TERMS AND CONDITIONS

    APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "{}"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

    Copyright {yyyy} {name of copyright owner}

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

/* 
    Made modifications:
    
    Added repository licence 

    line 221: use constants::{map}
    line 301: set_sample(&mut p_map, x_coord, y_coord, map::INITIAL_SQUARE_ALGORITHM);
    line 326: let scale = map::ROUGHNESS * size; 
    line 346: let scale = map::ROUGHNESS * size;
*/

use constants::{map};
use std::{f32};
use rand;
use rand::{Rng};
use num::{Num};
use num::{Integer};

type Buffer<T> = Vec<T>;

struct EnumeratePixelMap {
    x: u32,
    y: u32,
    size: u32,
}

impl Iterator for EnumeratePixelMap {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<(u32, u32)> {
        if self.x >= self.size {
            self.x = 0;
            self.y += 1;
        }
        let (x, y) = (self.x, self.y);

        self.x += 1;

        if self.y >= self.size {
            None
        } else {
            Some((x, y))
        }
    }
}

pub struct PixelMap<T: Copy + Num> {
    map: Buffer<T>,
    size: u32,
}

impl<T: Copy + Num> PixelMap<T> {
    fn max(&self) -> i32 {
        (self.size - 1) as i32
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> T {
        let x = x as usize;
        let y = y as usize;

        self.map[x + y * (self.size as usize)]
    }

    fn set_pixel(&mut self, x: u32, y: u32, value: T) {
        let x = x as usize;
        let y = y as usize;

        self.map[x + y * (self.size as usize)] = value;
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    fn enumerate_pixels(&self) -> EnumeratePixelMap {
        EnumeratePixelMap {
            x: 0,
            y: 0,
            size: self.size,
        }
    }
}

pub fn construct(detail: u32) -> PixelMap<f32> {
    let size: u32 = 2u32.pow(detail) + 1;
    let size_vec = size as usize;
    let mut p_map = PixelMap {
        map: vec![0.0f32; size_vec * size_vec],
        size: size,
    };

    for x_coord in vec![0, p_map.max()] {
        for y_coord in vec![0, p_map.max()] {
            set_sample(&mut p_map, x_coord, y_coord, map::INITIAL_SQUARE_ALGORITHM);
        }
    }

    let initial_size = p_map.max();

    divide(&mut p_map, initial_size);
    p_map
}

fn divide(p_map: &mut PixelMap<f32>, size: i32) {
    let half = size / 2;

    if half < 1 {
        return;
    }

    square_step(p_map, size);
    diamond_step(p_map, size);
    divide(p_map, size / 2);
}

fn diamond_step(p_map: &mut PixelMap<f32>, feature_size: i32) {
    let half_step = feature_size / 2;
    let size = feature_size as f32;
    let scale = map::ROUGHNESS * size;
    let mut y: i32 = 0;

    while y <= p_map.max() {
        let mut x: i32 = (y + half_step) % feature_size;
        while x <= p_map.max() {
            diamond_sample(p_map,
                           x,
                           y,
                           rand::thread_rng().next_f32() * scale * 2.0 - scale,
                           half_step);
            x += feature_size;
        }
        y += half_step;
    }
}

fn square_step(p_map: &mut PixelMap<f32>, feature_size: i32) {
    let half_step = feature_size / 2;
    let size = feature_size as f32;
    let scale = map::ROUGHNESS * size;
    let mut y = half_step;

    while y < p_map.max() {
        let mut x = half_step;

        while x < p_map.max() {
            square_sample(p_map,
                          x,
                          y,
                          rand::thread_rng().next_f32() * scale * 2.0 - scale,
                          half_step);
            x += feature_size;
        }
        y += feature_size;
    }
}

fn diamond_sample(p_map: &mut PixelMap<f32>, x: i32, y: i32, rand_value: f32, half_size: i32) {
    let a = sample(p_map, x, y - half_size);
    let b = sample(p_map, x - half_size, y);
    let c = sample(p_map, x + half_size, y);
    let d = sample(p_map, x, y + half_size);
    let value = (a + b + c + d) / 4.0 + rand_value;

    set_sample(p_map, x, y, value);
}

fn square_sample(p_map: &mut PixelMap<f32>, x: i32, y: i32, rand_value: f32, half_size: i32) {
    let a = sample(p_map, x - half_size, y - half_size);
    let b = sample(p_map, x + half_size, y - half_size);
    let c = sample(p_map, x - half_size, y + half_size);
    let d = sample(p_map, x + half_size, y + half_size);
    let value = (a + b + c + d) / 4.0 + rand_value;

    set_sample(p_map, x, y, value);
}

fn sample(p_map: &mut PixelMap<f32>, x: i32, y: i32) -> f32 {
    let grid_size = p_map.size as i32;
    let x = x.mod_floor(&grid_size) as u32;
    let y = y.mod_floor(&grid_size) as u32;

    p_map.get_pixel(x, y)
}

fn set_sample(p_map: &mut PixelMap<f32>, x: i32, y: i32, value: f32) {
    let x = x as u32;
    let y = y as u32;

    p_map.set_pixel(x, y, value)
}

pub fn normalize_pixel_map(p_map: PixelMap<f32>) -> PixelMap<u8> {
    let size = p_map.size() as usize;
    let mut new_p_map = PixelMap {
        map: vec![0u8; size* size],
        size: p_map.size(),
    };

    let max_value = max(&p_map);
    let min_value = min(&p_map);

    for (x, y) in p_map.enumerate_pixels() {
        let mut f_value = p_map.get_pixel(x, y);
        f_value -= min_value;
        f_value /= max_value - min_value;
        f_value *= u8::max_value() as f32;
        new_p_map.set_pixel(x, y, f_value as u8);
    }

    new_p_map
}

fn min(p_map: &PixelMap<f32>) -> f32 {
    let mut min_value = f32::MAX;

    for (x, y) in p_map.enumerate_pixels() {
        let f_value = p_map.get_pixel(x, y);
        if f_value < min_value {
            min_value = f_value;
        }
    }

    min_value
}

fn max(p_map: &PixelMap<f32>) -> f32 {
    let mut max_value = -f32::MAX;

    for (x, y) in p_map.enumerate_pixels() {
        let f_value = p_map.get_pixel(x, y);

        if f_value > max_value {
            max_value = f_value;
        }
    }

    max_value
}
