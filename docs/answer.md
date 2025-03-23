---
layout: default
title: Answer
---

# Answer

This document contains the expected results for all test cases, helping to verify the correctness of implementations. The results include the miss rate for each test case and the tags ordered by access frequency in each cache set.

<br>

## `test_50.txt`: Contains 50 lines of test addresses.

  - Cache Configuration:

    - Cache Size: 1 KBytes

    - Block Size: 16 words

    - Set Degree (Associativity): 2-way

<br>

Hits = 1, misses = 49, `miss rate = 0.98000`

<br>

Set num = 8

<br>

| Set Index |   Hit   |   Miss   |       Tags       |
|-----------|---------|----------|------------------|
|   Set 0   |    0    |     6    |[`5554766`, `5555228`, 5555229, 5555234, 5554924, 5554748]|
|   Set 1   |    1    |     5    |[`5555217`, `5554282`, 5554290, 5554738, 5554290, 5554246]|
|   Set 2   |    0    |     4    |[`5554761`, `5555238`, 5555203, 5555232]|
|   Set 3   |    0    |    10    |[`5554740`, `5554849`, 5554761, 5554293, 5554462, 5554790, 5554218, 5555216, 5555226, 5554733]|
|   Set 4   |    0    |     4    |[`5554747`, `5555228`, 5555223, 5554382]|
|   Set 5   |    0    |     6    |[`5555222`, `5554841`, 5554789, 5555203, 5554280, 5554484]|
|   Set 6   |    0    |     5    |[`5555215`, `5554315`, 5554720, 5554377, 5554279]|
|   Set 7   |    0    |     9    |[`5554721`, `5555200`, 5554912, 5554220, 5554244, 5554813, 5554211, 5554904, 5554409]|

<br>

Note: The most frequently accessed tags appear first in each set. The resident tags are `highlighted`.

<br>

---

## `test_100.txt`: Contains 100 lines of test addresses. <br><br>

- Cache Configuration:

    - Cache Size: 1 KBytes

    - Block Size: 4 words

    - Set Degree (Associativity): 2-way

<br>

Hits = 70, misses = 30, `miss rate = 0.30000`

<br>

Set num = 32

<br>

| Set Index |   Hit   |   Miss   |       Tags       |
|-----------|---------|----------|------------------|
|   Set 0   |    21   |    7     |[`665985`, 665985, 665985, 665985, `38965`, 38965, 38965, 38965, 643617, 643617, 643617, 643617, 394245, 394245, 394245, 394245, 485401, 485401, 485401, 485401, 677866, 677866, 677866, 677866, 393184, 393184, 393184, 393184]|
|   Set 1   |    21    |   7    |[`665985`, 665985, 665985, 665985, `38965`, 38965, 38965, 38965, 643617, 643617, 643617, 643617, 394245, 394245, 394245, 394245, 485401, 485401, 485401, 485401, 677866, 677866, 677866, 677866, 393184, 393184, 393184, 393184]|
|   Set 2   |     7   |    7    |[`665985`, 665985, `38965`, 38965, 643617, 643617, 394245, 394245, 485401, 485401, 677866, 677866, 393184, 393184]|
|   Set 3   |     0   |    0    |[]|
|   Set 4   |     0   |    0    |[]|
|   Set 5   |     0   |    0    |[]|
|   Set 6   |     0   |    0    |[]|
|   Set 7   |     0   |    0    |[]|
|   Set 8   |     0   |    0    |[]|
|   Set 9   |     0   |    0     |[]|
|   Set 10   |    0    |   0      |[]|
|   Set 11   |    0    |   0     |[]|
|   Set 12   |    0    |   0      |[]|
|   Set 13   |    0    |   0      |[]|
|   Set 14   |    0    |   0      |[]|
|   Set 15   |    0    |   0      |[]|
|   Set 16   |    9    |   3      |[`237236`, 237236, 237236, 237236, `430917`, 430917, 430917, 430917, 202620, 202620, 202620, 202620]|
|   Set 17   |    9    |   3      |[`237236`, 237236, 237236, 237236, `430917`, 430917, 430917, 430917, 202620, 202620, 202620, 202620]|
|   Set 18   |    3    |   3      |[`237236`, 237236, `430917`, 430917, 202620, 202620]|
|   Set 19   |    0    |   0      |[]|
|   Set 20   |    0    |   0      |[]|
|   Set 21   |    0    |   0      |[]|
|   Set 22   |    0    |   0      |[]|
|   Set 23   |    0    |   0      |[]|
|   Set 24   |    0    |   0      |[]|
|   Set 25   |    0    |   0      |[]|
|   Set 26   |    0    |   0      |[]|
|   Set 27   |    0    |   0      |[]|
|   Set 28   |    0    |   0      |[]|
|   Set 29   |    0    |   0      |[]|
|   Set 30   |    0    |   0      |[]|
|   Set 31   |    0    |   0      |[]|

<br>

Note: The most frequently accessed tags appear first in each set. The resident tags are `highlighted`.
