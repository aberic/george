# SIAM引擎
siam即static index access method，是一种静态索引方法，用于为数据库提供存取方法。该方法可以快速构建一个符合预期的树形结构，根据期望的层高设定对应的度值。<br>
**注：** 该索引支持的树形结构不受结构是逻辑限制，即b-tree或b+tree等都可适用。
<br><br>
## b-tree 模型
#### b-tree模型有以下概念：<br>
* 结点（node），表示树中的元素；<br>
* 结点的度（degree），拥有子结点的个数；<br>
* 叶子（leaf），度为0的结点；<br>
* 树的度，树中结点的最大的度；<br>
* 结点的层次（level），根结点是第一层，它的孩子结点是第二层，依次类推；<br>
* 树的高度，最大层次数。

#### SIAM介绍
在siam引擎中，每一个结点的度的最大值都是相等的，为了尽可能的降低IO读写次数，层高尽可能的设定在4及以下。
<br>
siam当前版本默认level均为4，在后续优化过程中会根据实际情况做相应调整，或开放出模型自定义构建接口。
<br>
在树形结构中，每一个结点中都包含有一个整数区间，同一层的结点与结点之间也存在间距，间距是根据前一结点区间的最大值与后一结点区间最小值差的绝对值。
<br>

#### SIAM模拟
参考`src/engine/comm`

SIAM文件格式
------------
#### 完整信息
| 首部信息 | 文件正文 |
| ------ | ------ |
| 32字节 | - |


----------


#### 首部信息
| 起始符 | 摘要信息 | 截止符 |
| ------ | ------ | ------ |
| 2字节 | 28字节 | 2字节 |
#### 起始符
| 固定标记1 | 固定标记2 |
| ------ | ------ |
| 0x20 | 0x19 |
#### 摘要信息
| 文件信息 | 保留占位 | 
| ------ | ------ |
| 7字节 | 21字节 |
#### 文件信息
| 标识符 | 存储类型 | 存储容量 | 索引类型 | 版本号 | 序号 | 
| ------ | ------ | ------ | ------ | ------ | ------ |
| 1字节 | 1字节 | 1字节 | 1字节 | 2字节 | 1字节 |
#### 截止符
| 固定标记1 | 固定标记2 |
| ------ | ------ |
| 0x02 | 0x19 |


----------

#### 文件正文
| 正文描述 | 正文内容|
| ------ | ------ |
| 6字节（index 8字节） | - |
#### 正文描述
| 描述起始位置 | 描述持续长度|
| ------ | ------ |
| 4字节 | 2字节（index 4字节） |
#### 正文内容
| 内容编码 |
| ------ |
| hex.encode |
