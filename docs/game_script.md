# x86 BIOS Simulator — 中英双语完整游戏剧本

> 基于 DIALOGUE_REFERENCE.md 编写，包含所有中文原文及完整英文翻译。
> 所有中文内容严格忠实于原文，零内容丢失。

---

## 一、画外音 / 玩家内心独白

### 1.1 开场独白（场景 1-1：整理遗物）

**场景：** 1-1
**触发条件：** 游戏开场，玩家进入阁楼房间

> 六月二十八号，我毕业了。
>
> 四年计算机系，从 C 语言到操作系统，从汇编到网络。
> 毕业证上写着"计算机科学与技术"。
>
> 但说实话，我从来没真正摸过一台老电脑。
> 我见过 BIOS 的界面，但那是图形化的 UEFI。
> 我写过汇编作业，但那是在模拟器里。
>
> 外公走得很突然。医生说是心脏的问题，但我知道，他心里一直有事。
>
> 我从没听他提起过辛巳游戏工作室的事。每次我问，他都只是摇摇头，说"过去的事了"。
>
> 现在我有几个月空闲时间，等九月入职。
> 我决定来整理他的阁楼。
>
> 也许……能发现一些什么。

**English:**

> June 28th. I graduated.
>
> Four years in the computer science department, from C language to operating systems, from assembly to networking.
> My diploma says "Computer Science and Technology."
>
> But honestly, I've never truly touched a real old computer.
> I've seen the BIOS screen, but that was the graphical UEFI.
> I wrote assembly homework, but that was in an emulator.
>
> Grandpa passed away suddenly. The doctors said it was his heart, but I know — there was something weighing on his mind.
>
> I never heard him mention the Xinsi Game Studio. Every time I asked, he'd just shake his head and say, "That's in the past."
>
> Now I have a few months of free time before I start work in September.
> I decided to clean up his attic.
>
> Maybe... I'll find something.

---

### 1.2 读取到扇区 200 后（第二章，场景 2-5）

**场景：** 2-5
**触发条件：** 读取硬盘扇区 200 后

> 三百万美元……
>
> 这不是什么"时间胶囊"研究经费。
>
> 这是……贪污？

**English:**

> Three million dollars...
>
> This isn't some "time capsule" research funding.
>
> This is... embezzlement?

---

### 1.3 发现邻居灯光闪烁后（第二章，场景 2-5）

**场景：** 2-5
**触发条件：** 观察到邻居窗户灯光闪烁

> 有人在看着这边？

**English:**

> Is someone watching?

---

### 1.4 解密证据后（第三章，场景 3-4）

**场景：** 3-4
**触发条件：** 成功解密证据文件

> 外公……
>
> 他不是在做什么秘密实验。
> 他是在收集证据，举报犯罪。
>
> "不要格式化 C 盘"……
> 原来是这个意思。
>
> 他把证据藏在了最不可能有人看到的地方——
> 硬盘的隐藏扇区里，用只有懂 BIOS 的人才能读取的方式。
>
> 我……我得把这份证据保存下来。

**English:**

> Grandpa...
>
> He wasn't conducting some secret experiment.
> He was collecting evidence, reporting a crime.
>
> "Do not format Drive C"...
>
> So that's what it meant.
>
> He hid the evidence in the last place anyone would think to look —
> in the hidden sectors of the hard drive, in a way only someone who understood BIOS could read.
>
> I... I have to preserve this evidence.

---

### 1.5 接到第一次威胁电话后（第二章）

**场景：** 第二章（接电话事件后）
**触发条件：** 接到李德胜第一次威胁电话后

> 他知道外公去世了……
> 他知道有人在用这台电脑……
> 他在监视我们？
>
> 但是他说"官司早就打完了"……
> 他已经服完刑了？
>
> 那他为什么还在害怕？
> 除非……还有他没被发现的罪行？
>
> 我不能停下来。

**English:**

> He knows Grandpa passed away...
> He knows someone is using this computer...
> Is he watching us?
>
> But he said "the lawsuit was settled long ago"...
> He's already served his time?
>
> Then why is he still afraid?
> Unless... there are crimes of his that haven't been discovered yet?
>
> I can't stop now.

---

### 1.6 接到第三次电话后（第四章）

**场景：** 第四章（第三次电话事件后）
**触发条件：** 接到李德胜第三次电话后

> 他慌了。
>
> "有人顶罪"——这说明他承认自己有罪。
> "那些钱我早就还了"——说明 300 万美元的事是真的。
>
> 但最重要的是最后一句——
> "里面有……"
> 他没说完。暗室里面有什么他这么害怕的？
>
> 我必须打开那个暗室。

**English:**

> He's panicking.
>
> "Someone took the fall" — that means he's admitting he's guilty.
> "I already paid back that money" — which means the three million dollars is real.
>
> But the most important thing is the last sentence —
> "Inside there's..."
>
> He didn't finish. What is in that secret room that scares him so much?
>
> I must open that room.

---

### 1.7 格式化 C 盘坏结局（特殊触发）

**场景：** 特殊触发
**触发条件：** 玩家选择格式化 C 盘

> 外公最后的话，永远消失了。
> 我格式化了一切，就像他从未存在过一样。

**English:**

> Grandpa's final words are gone forever.
> I formatted everything, as though he had never existed.

---

## 二、外公的声音（回忆/录音）

### 2.1 开场回忆（场景 1-1：整理遗物）

**场景：** 1-1
**触发条件：** 游戏开场，进入阁楼房间

*（轻柔地，像是老式录音机里传来的声音）*

> 孩子，如果你听到这段话，说明我不在了。
>
> 阁楼里有一台电脑。我留下来的东西，都在那里。
>
> 有些事情，我没法直接告诉你。但那台电脑……
>
> 它会告诉你一切。

**English:**

*(Gently, as if coming from an old tape recorder)*

> Child, if you're hearing this, it means I'm no longer here.
>
> There's a computer in the attic. Everything I left behind is there.
>
> There are things I couldn't tell you directly. But that computer...
>
> It will tell you everything.

---

### 2.2 发现软盘时的回忆（场景 1-3：发现软盘）

**场景：** 1-3
**触发条件：** 玩家找到软盘时

> 我在辛巳游戏工作室工作了十二年。
>
> 那时候，我们相信自己在做一件有意义的事。
>
> "时间胶囊"项目——我们想把人类的知识保存下来，
> 用最底层的方式，保存在磁盘的每一个扇区里。
>
> 后来我才发现，有些人想用它做完全不同的事。
>
> 这台电脑里，有他们不想让人知道的东西。

**English:**

> I worked at Xinsi Game Studio for twelve years.
>
> Back then, we believed we were doing something meaningful.
>
> The "Time Capsule" project — we wanted to preserve human knowledge,
> at the most fundamental level, stored in every sector of the disk.
>
> It wasn't until later that I discovered some people wanted to use it for something entirely different.
>
> Inside this computer, there are things they don't want anyone to know.

---

### 2.3 普通结局画外音（场景 4-3：显示图像）

**场景：** 4-3
**触发条件：** 达成普通结局，显示外公图像

> 孩子，我不知道你什么时候会看到这些。
>
> 但我相信，你总有一天会发现。
>
> 我当年没能公开这些证据——
> 有些人势力太大，我只能把它们藏起来。
>
> 但现在，你已经学会了使用这台电脑。
>
> 你学会了 BIOS，学会了操作磁盘，
> 学会了分析文件系统。
>
> 这些是最底层的知识，是最不会被人忘记的知识。
>
> 你做得很好。
>
> 带着这些证据，去做正确的事。
>
> —— 外公

**English:**

> Child, I don't know when you'll see these.
>
> But I believe that one day, you will find them.
>
> I couldn't make this evidence public back then —
> some people had too much power. I could only hide it.
>
> But now, you've learned to use this computer.
>
> You've learned BIOS, learned to operate disks,
> learned to analyze file systems.
>
> These are the most fundamental kinds of knowledge — the kind that's hardest to forget.
>
> You've done well.
>
> Take this evidence and do the right thing.
>
> — Grandpa

---

### 2.4 隐藏结局视频独白（场景 4-4：暗室）

**场景：** 4-4
**触发条件：** 达成隐藏结局，播放外公的告别视频

*（像素动画：外公坐在电脑前，对着镜头微笑）*

> 如果你看到这个视频，说明你找到了暗室。
>
> *（外公笑了一下）*
>
> 我就知道你能行。你从小就聪明。
>
> *（外公的表情变得严肃）*
>
> 这些年，我一直担心那些证据的安全。
>
> 李德胜虽然被判了刑，但他的同伙们还在外面。
> 他们一直在找这些证据，想彻底销毁。
>
> 但我把它们藏得很好。只有真正懂技术的人才能找到。
>
> *（外公看着镜头，眼神温柔）*
>
> 孩子，我不知道你什么时候会看到这个。
> 也许十年后，也许二十年后。
>
> 但知识是不会过时的。
> 你现在学会了这些东西，以后无论发生什么，
> 你都有能力保护自己和你爱的人。
>
> *（外公站起身，走到镜头前）*
>
> 好好照顾自己。
>
> 告诉你妈妈，我很想她。
>
> *（外公按下了停止键）*
>
> —— 外公，王志远
> 1998 年 6 月 15 日

**English:**

*(Pixel animation: Grandpa sits in front of the computer, smiling at the camera)*

> If you're watching this video, it means you found the secret room.
>
> *(Grandpa smiles)*
>
> I knew you could do it. You were always a smart kid.
>
> *(Grandpa's expression turns serious)*
>
> All these years, I've been worried about the safety of that evidence.
>
> Though Li Desheng was convicted, his accomplices are still out there.
> They've been searching for this evidence, trying to destroy it completely.
>
> But I hid it well. Only someone who truly understands the technology could find it.
>
> *(Grandpa looks at the camera, his eyes warm)*
>
> Child, I don't know when you'll see this.
> Maybe ten years from now, maybe twenty.
>
> But knowledge never becomes obsolete.
> You've learned these things now, so no matter what happens,
> you'll have the ability to protect yourself and the people you love.
>
> *(Grandpa stands up, walks toward the camera)*
>
> Take good care of yourself.
>
> Tell your mother I miss her very much.
>
> *(Grandpa presses the stop button)*
>
> — Grandpa, Wang Zhiyuan
> June 15, 1998

---

### 2.5 彩蛋结局外公语音（场景 4-5：时间胶囊激活）

**场景：** 4-5
**触发条件：** 达成彩蛋结局，时间胶囊激活

> 你以为这就结束了？
> 还有更多东西等着你去发现。

**English:**

> You think this is over?
> There's more out there waiting to be discovered.

---

## 三、信件 / 日记 / 文件文本

### 3.1 README.TXT（C 盘根目录）

**场景：** 第一章，启动电脑后查看 C 盘根目录
**触发条件：** 成功启动 DOS，查看 C 盘根目录下的 README.TXT

```
致使用这台电脑的人：

如果你能看到这段话，说明你已经成功启动了这台老电脑。
这台电脑里保存着一些重要的东西。

请仔细查看 DISK_01 软盘中的文件。
里面有一封信。

记得：不要格式化 C 盘。

                        —— 王志远
                        1998 年 6 月 15 日
```

**English:**

```
To whoever uses this computer:

If you can see this message, it means you have successfully
booted up this old computer.
There are some important things stored in this computer.

Please carefully check the files on floppy disk DISK_01.
There is a letter inside.

Remember: Do not format Drive C.

                        — Wang Zhiyuan
                        June 15, 1998
```

---

### 3.2 纸条内容（抽屉中）

**场景：** 第一章，打开书桌抽屉
**触发条件：** 点击书桌上层抽屉

```
不要格式化 C 盘。

                                    —— 外公
```

**English:**

```
Do not format Drive C.

                                    — Grandpa
```

---

### 3.3 LETTER.TXT（DISK_01 软盘）

**场景：** 第一章，读取 DISK_01 软盘
**触发条件：** 插入 DISK_01 软盘并查看文件

```
亲爱的孩子：

当你读到这封信时，我已经不在了。

我在辛巳游戏工作室工作了 12 年。那段时间，我们开发了一个
项目，代号叫做"时间胶囊"。这个项目本应该造福社会，
但是公司的管理层——算了，这些事还是以后再说。

我在这台电脑里藏了一些重要的证据。证据被分成了好几份，
分别保存在不同的地方。软盘和硬盘里都有。

你需要学会使用 BIOS 中断来读取这些数据。
每一个软盘里都有关于下一步的提示。

请相信我，你有能力找到真相。

                        爱你的外公
                        王志远
                        1998 年 6 月 15 日
```

**English:**

```
Dear child,

By the time you read this letter, I will be gone.

I worked at Xinsi Game Studio for 12 years. During that time,
we developed a project codenamed "Time Capsule." This project
was supposed to benefit society, but the company's management —
never mind, I'll save those things for later.

I've hidden some important evidence in this computer.
The evidence has been split into several pieces,
stored in different places. Both on floppy disks and the hard drive.

You will need to learn to use BIOS interrupts to read this data.
Each floppy disk contains clues about what to do next.

Please trust me — you have the ability to find the truth.

                        Your loving grandpa
                        Wang Zhiyuan
                        June 15, 1998
```

---

### 3.4 DIARY.TXT（DISK_01 软盘）

**场景：** 第一章，读取 DISK_01 软盘
**触发条件：** 插入 DISK_01 软盘，查看日记文件

```
1998 年 6 月 14 日

今天发现了不得了的事情。李总他们不是在开发什么
"时间胶囊"，他们在用这个项目做掩护，实际上是在——

[以下内容被涂改，无法辨认]

我得把证据保存下来。INT 13h 可以让我直接读写磁盘
的任意扇区。我要把证据藏在硬盘的隐藏扇区里。

但首先，我得确认 BIOS 的磁盘服务正常工作。
明天用中断 INT 13h 测试一下。
```

**English:**

```
June 14, 1998

Today I discovered something incredible. President Li and his
people aren't developing a "Time Capsule" — they're using this
project as cover, when actually they're —

[The following content is redacted and illegible]

I have to preserve the evidence. INT 13h lets me directly read
and write to any sector on the disk. I'll hide the evidence in
the hidden sectors of the hard drive.

But first, I need to confirm that the BIOS disk services are
working properly. I'll test with interrupt INT 13h tomorrow.
```

---

### 3.5 DISK_02 的 INTRO.TXT（磁盘结构入门）

**场景：** 第一章/第二章，读取 DISK_02 软盘
**触发条件：** 插入 DISK_02 软盘并查看文件

```
磁盘结构入门：

一个硬盘由很多扇区组成。每个扇区 512 字节。
扇区用 CHS（柱面-磁头-扇区）来定位：

  Cylinder (柱面): 0-1023
  Head (磁头):     0-254
  Sector (扇区):   1-63

也可以用 LBA（逻辑块地址）来表示：
  LBA = (C x H_max + H) x S_max + (S - 1)

记住这个公式，后面要用到。

                        —— 外公的笔记
```

**English:**

```
Introduction to Disk Structure:

A hard disk is composed of many sectors. Each sector is 512 bytes.
Sectors are located using CHS (Cylinder-Head-Sector):

  Cylinder:   0-1023
  Head:       0-254
  Sector:     1-63

They can also be represented using LBA (Logical Block Addressing):
  LBA = (C x H_max + H) x S_max + (S - 1)

Remember this formula — you'll need it later.

                        — Grandpa's notes
```

---

### 3.6 外公的工作日志（第三章，地下室）

**场景：** 第三章，地下室探索
**触发条件：** 进入地下室，找到外公的工作日志

```
1998 年 6 月 10 日

今天试了新的方法。我把关键证据分散到硬盘的不同分区，
每个分区用不同的文件系统（FAT12、FAT16），
这样即使有人恢复了主分区，也找不到隐藏分区。

主分区用标准 MBR，隐藏分区用自定义的分区类型代码。
分区表在扇区 0 的偏移 0x1BE 处。

如果有人想恢复这些数据，需要：
1. 读取 MBR
2. 分析分区表
3. 找到隐藏分区的起始扇区
4. 用对应的文件系统读取
```

**English:**

```
June 10, 1998

Today I tried a new method. I scattered the key evidence across
different partitions on the hard drive, each partition using a
different file system (FAT12, FAT16),
so even if someone recovers the main partition, they won't
find the hidden ones.

The main partition uses a standard MBR; the hidden partitions use
custom partition type codes.
The partition table is at offset 0x1BE in sector 0.

If anyone wants to recover this data, they would need to:
1. Read the MBR
2. Analyze the partition table
3. Find the starting sector of the hidden partition
4. Read it with the corresponding file system
```

---

### 3.7 EVIDENCE 文件（解密后，第三章场景 3-4）

**场景：** 3-4
**触发条件：** 成功解密证据文件

```
Project CAPSULE 证据文件
========================

1. 1998 年 1 月：李德胜要求将项目资金 300 万转入瑞士银行账户
   账号：CH-XXXX-XXXX-XXXX
   证据来源：财务部邮件转发

2. 1998 年 3 月：发现项目实际用于监控公民通讯（违法）
   "时间胶囊"项目名义上是知识存储
   实际上开发了通讯监控后门
   证据来源：代码审查记录

3. 1998 年 5 月：李德胜威胁所有知情员工
   "谁敢说出去，后果自负"
   小陈被调离核心岗位
   老王被要求签署保密协议

4. 1998 年 6 月 12 日：我决定保留证据并匿名举报
   将关键证据分散存储在硬盘隐藏扇区
   使用多种加密方式保护
   准备将副本寄给记者

签名：王志远
日期：1998 年 6 月 15 日

附注：
如果你读到这份文件，说明我的计划成功了。
请将这些证据交给可以信任的人。
不要让坏人逍遥法外。

                        —— 外公
```

**English:**

```
Project CAPSULE Evidence File
=============================

1. January 1998: Li Desheng demanded the transfer of 3 million
   in project funds to a Swiss bank account
   Account: CH-XXXX-XXXX-XXXX
   Evidence source: Forwarded email from the finance department

2. March 1998: Discovered that the project was actually used for
   monitoring citizens' communications (illegal)
   The "Time Capsule" project was nominally for knowledge storage
   but actually developed communications surveillance backdoors
   Evidence source: Code review records

3. May 1998: Li Desheng threatened all employees who knew the truth
   "Anyone who dares to speak up will face the consequences"
   Xiao Chen was reassigned from the core team
   Lao Wang was required to sign a non-disclosure agreement

4. June 12, 1998: I decided to preserve the evidence and report it anonymously
   Key evidence was scattered across hidden sectors on the hard drive
   Multiple encryption methods were used for protection
   Preparing to send copies to journalists

Signed: Wang Zhiyuan
Date: June 15, 1998

Note:
If you are reading this file, it means my plan succeeded.
Please hand this evidence to someone trustworthy.
Do not let bad people get away with it.

                        — Grandpa
```

---

### 3.8 FINAL.TXT（DISK_FINAL 软盘）

**场景：** 第四章，读取 DISK_FINAL 软盘
**触发条件：** 插入 DISK_FINAL 软盘并查看文件

```
孩子：

当你读到这封信的时候，你已经学会了很多东西。
你学会了 BIOS 中断，学会了操作磁盘，学会了
分析文件系统。这些是计算机最底层的知识。

我当年用这些技术保护了证据，让坏人受到了惩罚。
虽然我不能公开这些事情，但我知道有一天会有人
继承我的知识。

VIDEO.DAT 是一个用 BIOS 中断 INT 10h 的图形模式
绘制的图像。你可以用中断调试器在 320x200 的
256 色模式下显示它。

FINAL.DAT 是我留给你的礼物——一个完整的
BIOS 中断参考手册的手写扫描版。

你做得很好。

                        外公
```

**English:**

```
Child,

By the time you read this letter, you will have learned
many things. You've learned BIOS interrupts, learned to
operate disks, learned to analyze file systems. These are
the most fundamental knowledge of computers.

Back then, I used these technologies to protect the evidence
and bring criminals to justice. Although I couldn't make
these things public, I knew that one day someone would
carry on my knowledge.

VIDEO.DAT is an image drawn using the graphics mode of
BIOS interrupt INT 10h. You can display it using the
interrupt debugger in 320x200 256-color mode.

FINAL.DAT is my gift to you — a scanned hand-written
complete BIOS interrupt reference manual.

You've done well.

                        Grandpa
```

---

### 3.9 隐藏结局暗室电脑屏幕文字

**场景：** 4-4，隐藏结局暗室
**触发条件：** 打开暗室中的电脑

```
ACCESSING ENCRYPTED ARCHIVE...
DECRYPTING...
DECRYPTION KEY VERIFIED.
COMPLETE.

Hello, my grandchild.
You found the room.
I knew you would.

[播放视频]
```

**English:**

```
ACCESSING ENCRYPTED ARCHIVE...
DECRYPTING...
DECRYPTION KEY VERIFIED.
COMPLETE.

Hello, my grandchild.
You found the room.
I knew you would.

[Play video]
```

---

### 3.10 彩蛋结局屏幕文字（1998-06-15 启动）

**场景：** 彩蛋结局
**触发条件：** 在特定日期 1998-06-15 启动电脑

```
June 15, 1998

I am leaving this computer for the future.
I hope someone will find it and understand.

The BIOS holds more secrets than you think.
Keep exploring. Keep learning.

Time capsule activated.

                        — W.Z.Y.
                        1998-06-15
```

**English:**

```
June 15, 1998

I am leaving this computer for the future.
I hope someone will find it and understand.

The BIOS holds more secrets than you think.
Keep exploring. Keep learning.

Time capsule activated.

                        — W.Z.Y.
                        1998-06-15
```

---

### 3.11 张阿姨保管的信件（第三章，支线触发）

**场景：** 第三章，支线任务
**触发条件：** 触发张阿姨信封支线，获得信件

信件封面：

```
致我的后人

王志远
1998 年 6 月 15 日
```

**English:**

```
To my descendants

Wang Zhiyuan
June 15, 1998
```

信件正文：

```
亲爱的后人：

如果你读到这封信，说明你已经发现了电脑里的秘密。

但电脑里的证据并不完整。
我最信任的不是电脑——而是纸张。

这封信里有一把钥匙的图案。
用它打开阁楼书架后面的镜子。
那里有我留下的最后的东西。

这把钥匙的密码是：
你的生日（反转后）加上外公的生日。

不要忘记。

                        爱你的外公
                        王志远
```

**English:**

```
Dear descendant,

If you're reading this letter, it means you've discovered
the secrets hidden in the computer.

But the evidence in the computer is not complete.
What I trust most is not the computer — it's paper.

There is a key pattern in this letter.
Use it to open the mirror behind the bookshelf in the attic.
That's where I left the last thing.

The password for this key is:
Your birthday (reversed) plus Grandpa's birthday.

Don't forget.

                        Your loving grandpa
                        Wang Zhiyuan
```

---

## 四、张阿姨（邻居）对话

### 4.1 第一次见面（第二章，场景 2-1 后）

**场景：** 2-1 后
**触发条件：** 点击窗户

**对话：**

> "哎呀，你是老王家的孙子吧？
> 长这么大了……时间过得真快。"

**English:**

> "Oh my, you must be old Wang's grandson!
> You've grown so much... How time flies."

**玩家选择：**

- **A. "您认识我外公？"**
  > "认识啊，住了 20 多年的邻居了。你外公人好，就是不太爱说话。
  > 自从他从那个什么……辛巳游戏工作室辞职以后，就很少出门了。
  > 整天在阁楼里摆弄那台电脑。"

  **English:**

  > "Of course I do. We've been neighbors for over 20 years. Your grandpa
  > is a good man, just not much of a talker. Ever since he quit that...
  > what was it, the Xinsi Game Studio, he rarely went out.
  > He'd spend all day tinkering with that computer in the attic."

- **B. "您知道外公以前在做什么吗？"**
  > "辛巳游戏工作室啊，好像是搞什么……计算机研究的？
  > 我也不太懂这些。你外公从来不细说。
  > 不过有一年，他脸色特别差，好几天没出门。
  > 我去敲门，他说'没事，只是工作上的事'。"

  **English:**

  > "The Xinsi Game Studio? I think they did some kind of...
  > computer research? I don't really understand these things.
  > Your grandpa never went into detail.
  > But one year, he looked terrible — didn't go out for days.
  > I knocked on his door, and he said, 'It's nothing, just work stuff.'"

- **C. "（点头，没有说话）"**
  > "你看起来很像你外公年轻的时候。
  > 有空来阿姨家坐坐，我给你讲讲他以前的事。"

  **English:**

  > "You look just like your grandpa when he was young.
  > Come visit me sometime, and I'll tell you stories about him."

**笔记本更新：**
> 张阿姨是外公的老邻居。
> 她说外公从辛巳游戏工作室辞职后变得很少出门。
> 有一年外公脸色特别差——也许和 1998 年的事有关。

**Notebook update:**
> Aunt Zhang is Grandpa's old neighbor.
> She says Grandpa rarely went out after quitting from Xinsi Game Studio.
> One year Grandpa looked terrible — it might be related to the events of 1998.

---

### 4.2 第二次拜访（第二章，场景 2-3 后）

**场景：** 2-3 后
**触发条件：** 收集到至少 2 个数据碎片后，再次点击窗户

**对话：**

> "又在看对面啊？你外公以前也这样，
> 晚上一个人对着电脑发呆。"
>
> "对了，有件事我一直没说。"
>
> "1998 年有段时间，总有一个穿西装的人来找你外公。
> 看起来挺有钱的，但感觉不太友善。"

**English:**

> "Looking across again? Your grandpa used to do the same thing,
> staring at his computer alone at night."
>
> "Oh right, there's something I never told you."
>
> "Back in 1998, there was a man in a suit who kept coming to see
> your grandpa. He looked wealthy, but something about him
> didn't feel friendly."

**玩家选择：**

- **A. "那个人长什么样？"**
  > "很高，瘦瘦的，戴金丝眼镜。头发梳得一丝不苟。
  > 开一辆黑色的奔驰——那时候开奔驰的人可不多。
  >
  > 他每次来都很凶，说话声音很大。
  > 有一次我听到他喊：'王志远，你最好把东西交出来！'
  >
  > 我吓了一跳，但你外公把他关在门外，不让他进来。"

  **English:**

  > "Very tall, thin, wearing gold-rimmed glasses. Hair combed
  > immaculately. Drove a black Mercedes — not many people
  > drove Mercedes back then."
  >
  > "Every time he came he was aggressive, talked really loudly.
  > Once I heard him yell: 'Wang Zhiyuan, you'd better hand
  > over the stuff!'"
  >
  > "I was startled, but your grandpa locked him out and
  > wouldn't let him in."

- **B. "后来呢？发生了什么？"**
  > "后来？后来那个穿西装的人就没再来过了。
  > 但是过了几个月，你外公也辞职了。
  >
  > 再后来……辛巳游戏工作室就倒闭了。
  > 听说那个李总被抓了，好像是什么经济问题。
  >
  > 你外公从来没提过这事。他是个嘴巴很紧的人。"

  **English:**

  > "What happened next? Well, that man in the suit never came
  > again. But a few months later, your grandpa also quit."
  >
  > "And then... the Xinsi Game Studio shut down.
  > I heard that President Li got arrested, something about
  > financial crimes."
  >
  > "Your grandpa never talked about it. He was a man who
  > kept his mouth shut."

- **C. "（翻看笔记本中的公司资料）这个人是李德胜吗？"**
  > "对对对，就是这个名字！李……李什么来着？
  > 你怎么知道的？
  >
  > ……你外公留下的东西里有他的信息？
  > 孩子，有些事还是不要挖太深的好。
  > 你外公当年就是因为挖太深，才……"
  >
  > **张阿姨欲言又止，转移话题：**
  >
  > "今天天气不错啊，你要多出去走走，别老闷在阁楼里。
  > 你外公就是闷坏了，身体才出问题的。"

  **English:**

  > "Yes, yes, that's the name! Li... Li something?
  > How do you know that?"
  >
  > "...It's in the things your grandpa left behind?
  > Child, some things are better left uninvestigated.
  > Your grandpa dug too deep back then, and that's why he..."
  >
  > **Aunt Zhang stops herself mid-sentence and changes the subject:**
  >
  > "The weather is nice today. You should go out for a walk
  > more, don't just cooped up in the attic all day.
  > Your grandpa got sick from staying indoors too much."

**笔记本更新：**
> 关键信息！
> 1998 年，一个穿西装的人多次来找外公。
> 张阿姨描述的人符合：高、瘦、金丝眼镜、黑色奔驰。
> 外公对他很警惕，把他关在门外。
>
> 这个人大概率就是辛巳游戏工作室 CEO 李德胜。
> 他来找外公是为了……要回证据？

**Notebook update:**
> Key information!
> In 1998, a man in a suit repeatedly came to see Grandpa.
> The person Aunt Zhang described matches: tall, thin, gold-rimmed glasses, black Mercedes.
> Grandpa was wary of him and locked him out.
>
> This person is most likely Xinsi Game Studio CEO Li Desheng.
> He came to see Grandpa to... get the evidence back?

---

### 4.3 第三次拜访（第三章，场景 3-4 后）

**场景：** 3-4 后
**触发条件：** 解密证据后，再次点击窗户

**对话：**

> "孩子，我看得出来你心事重重。
> 是不是发现什么了？"

**English:**

> "Child, I can see something is weighing on your mind.
> Did you find something?"

**玩家选择：**

- **A. "我发现了外公当年调查的真相……"**
  > *张阿姨沉默了一会儿，叹了口气*
  >
  > "我就知道你早晚会发现的。
  > 你外公和你一样，聪明，又倔。
  >
  > 那年他跟我说：'张姐，如果有一天我不在了，
  > 有个孩子来找你，你就告诉他——
  > 外公做的事是对的。不要怀疑。'
  >
  > 我当时以为他只是说说……没想到他真的把证据留到了现在。"

  **English:**

  > *Aunt Zhang is silent for a moment, then sighs*
  >
  > "I knew you'd find out sooner or later.
  > Your grandpa was like you — smart and stubborn."
  >
  > "That year he told me: 'Sister Zhang, if one day I'm no longer
  > here, and a child comes looking for you, tell them —
  > what Grandpa did was right. Don't doubt it.'"
  >
  > "I thought he was just saying that... I never imagined he'd
  > actually kept the evidence all this time."

- **B. "您是不是还有什么没告诉我？"**
  > *张阿姨犹豫了一下*
  >
  > "你外公……其实来找过我一次。
  > 那是 1998 年的某一天，很晚了，大概凌晨两三点。
  >
  > 他把一个信封交给我，说：'帮我保管这个。
  > 如果 20 年后我还在，就还给我。
  > 如果我不在了……就等我的后人来取。'
  >
  > 我……我一直留着那个信封。在我家的保险柜里。
  > 你想看吗？"
  >
  > **（触发支线任务：张阿姨的信封）**

  **English:**

  > *Aunt Zhang hesitates*
  >
  > "Your grandpa... actually came to see me once.
  > It was one night in 1998, very late, around two or three in the morning."
  >
  > "He handed me an envelope and said: 'Help me keep this.
  > If I'm still here in 20 years, give it back to me.
  > If I'm no longer here... wait for my descendants to come get it.'"
  >
  > "I... I've kept that envelope all this time. In my
  > home safe. Would you like to see it?"
  >
  > **(Triggers side quest: Aunt Zhang's Envelope)**

  ---

  **（在 B/C 选择之后触发的额外对话：张阿姨的独白）**

  *（无论选择 B 或 C，触发信封支线后，张阿姨会额外说一段话）*

  > "你可能在想，我为什么愿意帮一个老头子守了二十多年的秘密。"
  >
  > "1989 年的时候，我丈夫生了一场大病。那时候医疗费要三万多块，
  > 我们家拿不出来。我到处借钱，没有人愿意帮我。"
  >
  > "是你外公。他二话不说，把积蓄拿了出来，借给了我。
  > 我说写借条，他说不用。
  > 他说：'张姐，邻居之间不用这么客气。以后你帮我看着这间房子就行。'"
  >
  > "后来我丈夫还是走了。但那笔钱，我省吃俭用，花了五年才还清。
  > 你外公收了钱，但转头就给我家孩子包了个红包。"
  >
  > "所以当他凌晨来找我，把那个信封交给我的时候，
  > 我没有问里面是什么。我只知道——
  > 这个人值得我为他做任何事。"

  **English:**

  > "You might be wondering why I was willing to keep an old man's
  > secret for over twenty years."
  >
  > "In 1989, my husband fell very ill. The medical bills were over
  > thirty thousand yuan — more than we could afford. I went everywhere
  > asking for help. No one was willing."
  >
  > "It was your grandpa. Without a second thought, he took out his
  > savings and lent them to me. I offered to write an IOU, but he
  > said no. He said: 'Sister Zhang, neighbors don't need to be so
  > formal. Just watch over my house for me.'"
  >
  > "My husband passed away eventually. But I saved every penny and
  > spent five years paying back that debt. Your grandpa took the
  > money, but turned around and gave my child a red envelope."
  >
  > "So when he came to me at three in the morning with that envelope,
  > I didn't ask what was inside. I only knew — this is someone I
  > would do anything for."

- **C. "我想问您一件事——外公有没有留信给您？"**
  > "……你怎么知道的？
  >
  > 是的，他留了一封信。
  > 但我答应过他，只有他的后人亲自来问，我才能拿出来。
  >
  > 你就是他的后人。跟我来吧。"

  **English:**

  > "...How did you know?"
  >
  > "Yes, he left a letter with me.
  > But I promised him I'd only take it out if his descendant
  > came to ask for it in person."
  >
  > "You are his descendant. Come with me."

**信件内容（触发支线后获得）：**

```
亲爱的后人：

如果你读到这封信，说明你已经发现了电脑里的秘密。

但电脑里的证据并不完整。
我最信任的不是电脑——而是纸张。

这封信里有一把钥匙的图案。
用它打开阁楼书架后面的镜子。
那里有我留下的最后的东西。

这把钥匙的密码是：
你的生日（反转后）加上外公的生日。

不要忘记。

                        爱你的外公
                        王志远
```

**English:**

```
Dear descendant,

If you're reading this letter, it means you've discovered
the secrets hidden in the computer.

But the evidence in the computer is not complete.
What I trust most is not the computer — it's paper.

There is a key pattern in this letter.
Use it to open the mirror behind the bookshelf in the attic.
That's where I left the last thing.

The password for this key is:
Your birthday (reversed) plus Grandpa's birthday.

Don't forget.

                        Your loving grandpa
                        Wang Zhiyuan
```

---

### 4.4 第四次拜访（隐藏结局达成后）

**场景：** 第四章后
**触发条件：** 达成隐藏结局后，再次点击窗户

**对话：**

> "孩子，我看到你房间的灯亮了一整晚。"
>
> "你外公……他一定很欣慰。"
>
> "他等了这么多年，就是在等你。"
>
> "好好保重自己。你外公会为你骄傲的。"

**English:**

> "Child, I saw the light in your room was on all night."
>
> "Your grandpa... he must be so proud."
>
> "He waited all these years, just for you."
>
> "Take good care of yourself. Your grandpa would be proud of you."

**成就解锁：**
```
老邻居
"张阿姨和外公做了 20 多年邻居。
 她一直在等你来。"
```

**Achievement unlocked:**
```
Old Neighbor
"Aunt Zhang and Grandpa were neighbors for over 20 years.
 She was waiting for you all along."
```

---

## 五、李德胜（李总）电话对话

### 5.1 第一次来电（第二章，收集到第 3 个数据碎片后）

**场景：** 第二章
**触发条件：** 读取扇区 200 后，电话响起

**对话：**

*（经过录音处理，声音有些沙哑和失真）*

> "王志远，你不用躲了。"
>
> "我知道你把东西藏在哪里。磁盘？扇区？
> 这些小把戏瞒不了我。"
>
> "我给你最后一次机会——
> 把东西交出来，我们一笔勾销。
> 否则……"
>
> *（录音中断，电话里传来忙音）*

**English:**

*(Processed through recording, the voice is somewhat hoarse and distorted)*

> "Wang Zhiyuan, there's no point hiding."
>
> "I know where you've hidden the stuff. Disk? Sectors?
> These little tricks won't fool me."
>
> "I'm giving you one last chance —
> hand over the goods, and we'll call it even.
> Otherwise..."
>
> *(The recording cuts off, a busy signal comes through the phone)*

**玩家选择：**

- **A. "（挂断电话，心跳加速）"**
- **B. "（查看来电显示——没有号码）"**
- **C. "（记录下录音的内容到笔记本）"**

**English:**

- **A. "(Hangs up, heart racing)"**
- **B. "(Checks caller ID — no number)"**
- **C. "(Records the content of the recording in the notebook)"**

**笔记本更新：**
> 刚接到了一个奇怪的电话。
> 录音中的声音威胁外公交出"东西"。
> 这个人很可能就是李德胜。
>
> 他说他知道证据藏在哪里……
> 难道他一直监视着这栋房子？
>
> 我需要加快速度。

**Notebook update:**
> Just received a strange phone call.
> The voice in the recording threatened Grandpa to hand over "the stuff."
> This person is very likely Li Desheng.
>
> He says he knows where the evidence is hidden...
> Has he been watching this house all along?
>
> I need to speed things up.

---

### 5.1b 环境事件：深夜异响（第一通电话后）

**场景：** 第二章，第一通电话之后
**触发条件：** 第一通电话结束后的当晚

*（画面变暗，CRT 屏幕的微光照亮阁楼。窗外是深夜。）*

**事件一：窗外的影子**

> 凌晨两点。我睡不着，坐在电脑前整理今天的发现。
>
> 窗外忽然闪过一道光——像是手电筒的光束，扫过窗户后消失了。
>
> 我站起来，走到窗边。对面张阿姨家的灯已经灭了。
> 街道上空无一人。
>
> 但路边停着一辆黑色的轿车。我之前没注意到。
> 车里有人吗？我看不到。

**笔记本更新：**
> 凌晨两点，窗外有手电筒的光闪过。
> 街道上停着一辆黑色轿车，之前没见过。
>
> 是李德胜的人？还是我多想了？
>
> 不管怎样，我把窗帘拉上了。

---

**事件二：被翻动的抽屉**

> 第二天早上，我下楼去买早餐。
> 回来时发现——
>
> 书桌的抽屉微微开着。我记得昨天关好了。
>
> 里面的东西没有少，但是……软盘的位置变了。
> DISK_01 从抽屉左边移到了右边。
>
> 有人进来过？
> 我检查了门锁——没有被撬的痕迹。
>
> 但是窗户的插销……好像没有扣紧。

**笔记本更新：**
> 有人翻过我的抽屉。
> 软盘被移动了，但没有被拿走。
>
> 他知道我在找什么，但不知道具体是哪些文件。
> 他在确认我有没有找到关键证据。
>
> 我必须更小心了。从今天起，每次离开房间都要拍照记录物品位置。

**English:**

> 2 AM. I can't sleep, sitting at the computer organizing today's findings.
>
> Suddenly a light flashes outside the window — like a flashlight beam, sweeping across the window and disappearing.
>
> I stand up and walk to the window. Aunt Zhang's lights across the street are already off. The street is empty.
>
> But there's a black sedan parked on the road that I didn't notice before. Is someone in the car? I can't tell.

**Notebook update:**
> 2 AM — flashlight beam outside the window. A black sedan on the street that wasn't there before.
> Is it Li Desheng's people? Or am I imagining things?
> Either way, I pulled the curtains shut.

---

**Notebook update:**
> Someone went through my drawers. The floppy disks were moved but not taken.
> He knows what I'm looking for, but not which specific files.
> I need to be more careful. Photograph the room's item positions every time I leave.

---

### 5.2 第二次来电（第三章，硬盘修复后）

**场景：** 第三章
**触发条件：** 成功修复 MBR 后

**对话：**

*（这次声音更加冷静，甚至带点笑意）*

> "王志远……哦不对，你应该不是王志远。"
>
> "我查过了，那个老头子三年前就死了。"
> "你是他的孙子？还是外孙？"
>
> "小朋友，听叔叔一句话——
> 有些事情不是你该管的。"
>
> "你以为你在做什么？正义使者？"
> "那些证据能改变什么？官司早就打完了。"
> "我早就出来了。我现在过得很好。"
>
> "但如果你非要挖下去……"
> "你确定你准备好了吗？"
>
> *（电话挂断）*

**English:**

*(This time the voice is much calmer, almost amused)*

> "Wang Zhiyuan... oh wait, you're probably not Wang Zhiyuan."
>
> "I looked it up. That old man died three years ago."
> "Are you his grandson? Or granddaughter?"
>
> "Kid, let me give you some advice —
> there are things that aren't your business."
>
> "Do you think you're some kind of justice warrior?"
> "What can that evidence change? The lawsuit was settled long ago."
> "I'm already out. I'm doing very well now."
>
> "But if you insist on digging..."
> "Are you sure you're ready for this?"
>
> *(Phone hangs up)*

---

### 5.2b 环境事件：被跟踪的迹象（第二通电话后）

**场景：** 第三章，第二通电话之后
**触发条件：** 第二通电话结束后

**事件一：门缝下的纸条**

> 今天出门买电池时，注意到街角那辆黑色轿车还在。
> 不是之前的那辆——这辆是银灰色的，但停的位置几乎一样。
>
> 我假装没看到，走进了便利店。
>
> 回来时，门口的地上有一张纸条。

**纸条内容：**

```
小朋友，游戏到此为止。
你的外公是个聪明人——但他不够聪明。
你比他更不够聪明。

把东西交出来，我可以保证你和你的家人都安全。
否则，下一次来的就不是纸条了。

——你的朋友
```

**笔记本更新：**
> 他给我留了纸条。就在我的门口。
>
> 这不是电话里的匿名威胁了——这是直接的、面对面的恐吓。
> 他知道我长什么样，知道我住在哪里，知道我每天几点出门。
>
> 他说"你的家人"——他在威胁我妈妈？
>
> 我需要报警吗？但如果报警，外公的证据怎么办？
>
> ……我先把纸条收好了。拍照留证据。

---

**事件二：电脑里的异常**

> 今天开机时，BIOS 的时间不对。
> 系统时间显示的是三天前的日期。
>
> 我没有动过 BIOS 设置。有人远程改了？
> 不对，这是一台没有联网的老电脑。不可能远程修改。
>
> 那就是有人物理接触过这台电脑。
> 在我不在的时候。
>
> 我检查了 BIOS 设置——启动顺序没变，其他参数也没变。
> 硬盘的 SMART 状态也正常。
>
> 但是……CMOS 电池旁边的灰尘有一个指纹印。
>
> 有人打开过机箱。

**笔记本更新：**
> BIOS 时间被改了。有人在我离开时进来过。
> 他们可能试图直接读取硬盘——但外公的证据藏在隐藏扇区，
> 只有通过 INT 13h 才能访问。普通的文件浏览器找不到。
>
> 这说明两件事：
> 1. 李德胜的人有物理接触这台电脑的能力
> 2. 但他们不懂 BIOS 中断——否则不会只是改了 CMOS 时间就走
>
> 我需要把关键数据备份出来。同时，加快进度。

**English:**

**Notebook update:**
> He left me a note. Right at my door.
> This isn't anonymous phone threats anymore — it's direct, face-to-face intimidation.
> He knows what I look like, where I live, my daily schedule.
>
> He mentioned "your family" — is he threatening my mother?
> Should I call the police? But then what about Grandpa's evidence?
>
> ...I kept the note. Photographed it as evidence.

---

**Notebook update:**
> BIOS time was changed. Someone accessed the computer while I was away.
> They probably tried to read the hard drive directly — but Grandpa's evidence is hidden in sectors only accessible through INT 13h. A normal file browser can't find it.
>
> Two things this tells me:
> 1. Li Desheng's people have physical access to this house
> 2. But they don't understand BIOS interrupts — otherwise they wouldn't have just changed CMOS time and left
>
> I need to back up the critical data immediately. And speed up my progress.

---

### 5.3 第三次来电（第四章，找到所有密码后）

**场景：** 第四章
**触发条件：** 找到地下室暗格密码后

**对话：**

*（这次声音明显紧张了）*

> "你找到了什么？"
>
> "我警告你，把那些东西删掉。"
> "你不知道你在惹谁。"
>
> "那些事情已经过去 20 多年了！"
> "谁还在乎？"
> "那些钱我早就还了！项目的事也有人顶罪！"
>
> "你……你别以为我不知道你在做什么。"
> "那个暗室……你不能打开它。"
> "里面有……"
>
> *（电话里传来一阵电流声，然后是长时间的沉默）*
>
> "算你狠。"
>
> *（电话挂断）*

**English:**

*(This time the voice is clearly nervous)*

> "What did you find?"
>
> "I'm warning you, delete those files."
> "You don't know who you're messing with."
>
> "Those things happened over 20 years ago!"
> "Who even cares anymore?"
> "I already paid back the money! Someone else took the fall for the project!"
>
> "You... don't think I don't know what you're doing."
> "That secret room... you can't open it."
> "Inside there's..."
>
> *(A burst of static comes through the phone, followed by a long silence)*
>
> "You've got guts."
>
> *(Phone hangs up)*

**笔记本更新：**
> 李德胜承认了：
> 1. 项目资金确实是被他挪用的
> 2. 有人替他顶了罪
> 3. 他一直在监视这栋房子
> 4. 暗室里有他害怕的东西
>
> 外公不只是藏了证据。
> 他藏的是能让李德胜再次入狱的证据。
>
> 我必须打开暗室。

**Notebook update:**
> Li Desheng admitted to:
> 1. The project funds were indeed embezzled by him
> 2. Someone else took the fall for him
> 3. He's been watching this house all along
> 4. There's something in the secret room that scares him
>
> Grandpa didn't just hide evidence.
> He hid evidence that could put Li Desheng back in prison.
>
> I must open that secret room.

---

## 六、录音带完整内容

### 6.1 录音带 #1 — 外公讲述加入辛巳游戏工作室的经历

**场景：** 第一章/第二章
**触发条件：** 找到录音带 #1
**位置：** 书架后面

*（年轻的外公，语气轻快）*

> "1986 年。我刚从大学毕业，学的是计算机工程。
> 那时候会用电脑的人不多，找工作特别容易。
>
> 辛巳游戏工作室是当时新兴的高科技公司。
> 他们的口号是'用科技改变未来'。
>
> 我第一天上班，李德胜亲自接见了我。
> 他说：'王志远，你是我见过的最有才华的年轻人。
>  从今天起，你就是我们团队的核心成员。'
>
> 当时我真的很激动。
> 我以为自己找到了人生的使命。
>
> ……现在想想，那些话只是客套。
> 他对每个人都是这么说的。"

**English:**

*(Young Grandpa, cheerful tone)*

> "1986. I had just graduated from college, majoring in computer
> engineering. Not many people knew how to use computers back then,
> so finding a job was easy."
>
> "Xinsi Game Studio was a new high-tech company at the time.
> Their slogan was 'Changing the future with technology.'"
>
> "My first day at work, Li Desheng personally welcomed me.
> He said: 'Wang Zhiyuan, you're the most talented young person
> I've ever met. From today on, you're a core member of our team.'"
>
> "I was truly thrilled at the time.
> I thought I'd found my life's mission."
>
> "...Looking back now, those were just pleasantries.
> He said the same thing to everyone."

---

### 6.2 录音带 #2 — "时间胶囊"项目的最初构想

**场景：** 第二章
**触发条件：** 找到录音带 #2
**位置：** 地下室纸箱

*（充满热情）*

> "'时间胶囊'——这是我起的名字。
>
> 我们想做一件前所未有的事：
> 把人类的知识，用最底层的方式，保存在磁盘里。
>
> 不是云存储，不是数据库，不是什么花哨的东西。
> 就是磁盘的每一个扇区，每一个字节。
>
> 这样即使所有的服务器都崩溃了，
> 即使所有的网络都断了，
> 只要有一台老电脑和一张软盘，
> 人类的知识就能传承下去。
>
> 当然，这只是一个开始。
> 我们真正的目标是……"
>
> *（外公停顿了一下）*
>
> "算了，这个以后再说。"

**English:**

*(Full of enthusiasm)*

> "'Time Capsule' — that's the name I came up with."
>
> "We wanted to do something unprecedented:
> preserve human knowledge in the most fundamental way,
> stored on disk."
>
> "Not cloud storage, not databases, not anything fancy.
> Just every sector, every byte on the disk."
>
> "That way, even if all servers crash,
> even if all networks go down,
> as long as there's one old computer and one floppy disk,
> human knowledge can be passed down."
>
> "Of course, this was just the beginning.
> Our real goal was..."
>
> *(Grandpa pauses)*
>
> "Never mind, I'll talk about that later."

---

### 6.3 录音带 #3 — 发现李总的真实意图

**场景：** 第二章
**触发条件：** 找到录音带 #3
**位置：** 软盘堆底部

*（语气沉重）*

> "今天我发现了一些不得了的事情。
>
> 项目资金的去向不对。
> 300 万——说是要买服务器和存储设备——
> 但我查了采购清单，那些设备根本不存在。
>
> 我去找李德胜，他笑着说：'老王，财务的事你不用操心。
> 专心搞你的技术就行。'
>
> 但我不放心。我开始调查资金流向。
>
> 结果……
>
> 钱被转到了瑞士银行的一个账户。
> 账户名是：李德胜。
>
> 这不是什么'时间胶囊'。
> 这是……贪污。"

**English:**

*(Somber tone)*

> "Today I discovered something incredible."
>
> "The project funds don't add up.
> Three million — supposedly to buy servers and storage
> equipment — but I checked the purchase orders,
> and those devices don't exist."
>
> "I went to Li Desheng, and he said with a smile: 'Lao Wang,
> don't worry about financial matters. Just focus on your
> technical work.'"
>
> "But I wasn't comfortable. I started investigating
> the flow of funds."
>
> "And the result..."
>
> "The money was transferred to a Swiss bank account.
> The account holder's name is: Li Desheng."
>
> "This isn't some 'Time Capsule.'
> This is... embezzlement."

---

### 6.4 录音带 #4 — 如何隐藏证据的计划

**场景：** 第三章
**触发条件：** 找到录音带 #4
**位置：** 机箱内部

*（压低声音，像是在密谋）*

> "我决定收集证据，然后举报。
> 但李德胜势力很大，我必须把证据藏好。
>
> 我想到了一个办法——
> 硬盘有很多隐藏的扇区，一般人根本不会去读。
>
> 我用 BIOS 中断 INT 13h，
> 把证据分散存储在硬盘的不同位置。
> 每个部分都用了不同的加密方式。
>
> 就算有人把硬盘拆走，格式化，重新分区，
> 也找不到完整的证据。
>
> 因为证据不在'文件'里——
> 它藏在文件系统的缝隙中。
>
> 这是只有懂 BIOS 的人才能发现的秘密。"

**English:**

*(Lowered voice, as if conspiring)*

> "I decided to collect evidence and report it.
> But Li Desheng has a lot of power, so I must hide the
> evidence well."
>
> "I thought of a way —
> the hard drive has many hidden sectors that ordinary people
> would never think to read."
>
> "Using BIOS interrupt INT 13h,
> I scattered the evidence across different locations on
> the hard drive. Each part uses a different encryption method."
>
> "Even if someone pulls out the hard drive, formats it,
> and repartitions it, they won't find the complete evidence."
>
> "Because the evidence isn't in any 'file' —
> it's hidden in the gaps of the file system."
>
> "This is a secret only someone who understands BIOS
> can discover."

---

### 6.5 录音带 #5 — 对家人的最后留言

**场景：** 第四章
**触发条件：** 找到录音带 #5
**位置：** 日历后面

*（温柔而疲惫）*

> "今天是 6 月 14 日。明天就是最后期限了。
>
> 我已经把所有证据都藏好了。
> 软盘、硬盘、地下室……每一处都有线索。
>
> 我最放心不下的，是我的家人。
>
> 告诉你妈妈，我很想她。
> 告诉你……如果你听到了这段话的话——
>
> 我不是什么英雄，也不是什么罪犯。
> 我只是一个想做正确的事的普通人。
>
> 也许这些证据永远不会被发现。
> 也许有一天，会有人用这台电脑，
> 学会 BIOS 中断，学会操作磁盘，
> 然后找到我藏起来的一切。
>
> 我相信那一天会到来的。
> 因为知识是不会消失的。"

**English:**

*(Gentle and weary)*

> "Today is June 14th. Tomorrow is the deadline."
>
> "I've hidden all the evidence.
> Floppy disks, the hard drive, the basement... there are clues
> in every place."
>
> "The thing I worry about most is my family."
>
> "Tell your mother I miss her very much.
> Tell you... if you're hearing this —"
>
> "I'm not some hero, nor am I some criminal.
> I'm just an ordinary person trying to do the right thing."
>
> "Maybe this evidence will never be found.
> Maybe one day, someone will use this computer,
> learn BIOS interrupts, learn to operate disks,
> and then find everything I've hidden."
>
> "I believe that day will come.
> Because knowledge never disappears."

---

### 6.6 录音带 #6 — 录制告别视频的幕后花絮

**场景：** 第四章，隐藏结局
**触发条件：** 找到录音带 #6
**位置：** 暗室

*（轻松、带着笑意）*

> "哈哈哈，这个角度太严肃了……
>
> 等一下，让我重新录。
>
> 嗯……如果你看到这个……
>
> 不对不对，太正式了。
>
> 好吧，就用最简单的话：
>
>  '孩子，你做到了。'
>
> 就这样。
> 不用说太多。
> 相信你的人，自然会懂。"
>
> *（外公轻笑）*
>
> "好了，录完了。
> 这台电脑会保存这个视频，直到有人打开它。
>
> ……再见了，我的老朋友。"

**English:**

*(Relaxed, with a smile in the voice)*

> "Hahaha, this angle is way too serious..."
>
> "Wait a moment, let me redo it."
>
> "Um... if you're watching this..."
>
> "No no, too formal."
>
> "Alright, let me just use the simplest words:"
>
> "'Child, you did it.'"
>
> "That's it.
> No need to say too much.
> The people who trust you will understand."
>
> *(Grandpa chuckles softly)*
>
> "Okay, that's a wrap.
> This computer will keep this video until someone turns it on."
>
> "...Goodbye, my old friend."

---

## 七、老照片语音内容

### 7.1 照片 #1 — 外公年轻时的证件照

**场景：** 第一章/第二章
**触发条件：** 查看照片 #1

> "这是 1985 年拍的。那时候我还不到 30 岁，对未来充满期待。"

**English:**

> "This was taken in 1985. I wasn't even 30 yet, full of hope for the future."

---

### 7.2 照片 #2 — 辛巳游戏工作室开业典礼

**场景：** 第一章/第二章
**触发条件：** 查看照片 #2

> "公司开业那天，李德胜请了很多记者。他站在最中间，笑容灿烂。"

**English:**

> "On the day the company opened, Li Desheng invited many reporters. He stood right in the middle, beaming."

---

### 7.3 照片 #3 — 团队合影

**场景：** 第一章/第二章
**触发条件：** 查看照片 #3

> "从左到右：我、小陈、老王、李德胜……那时候大家真的像一家人。"

**English:**

> "From left to right: me, Xiao Chen, Lao Wang, Li Desheng... Back then we really were like a family."

---

### 7.4 照片 #4 — "时间胶囊"项目发布会

**场景：** 第二章
**触发条件：** 查看照片 #4

> "这是我们最风光的时刻。所有人都相信我们在改变世界。"

**English:**

> "This was our proudest moment. Everyone believed we were changing the world."

---

### 7.5 照片 #5 — 外公和女儿在公园

**场景：** 第一章/第二章
**触发条件：** 查看照片 #5

> "这是你妈妈 10 岁生日。她最喜欢荡秋千了。"

**English:**

> "This is your mother's 10th birthday. She loved swinging on the swings."

---

### 7.6 照片 #6 — 外公的书房

**场景：** 第一章/第二章
**触发条件：** 查看照片 #6

> "这间书房后来变成了阁楼。你看到的那台电脑，就在这个位置。"

**English:**

> "This study later became the attic. The computer you see is right in this spot."

---

### 7.7 照片 #7 — 李德胜和外公的合影

**场景：** 第二章/第三章
**触发条件：** 查看照片 #7

> "你看李德胜的笑容……现在想想，他的笑从来没到过眼睛。"

**English:**

> "Look at Li Desheng's smile... thinking back now, his smile never reached his eyes."

---

### 7.8 照片 #8 — 外公独自坐在电脑前

**场景：** 第三章/第四章
**触发条件：** 查看照片 #8

> "这是最后一张照片。之后，我就不再拍照了。有些事，只适合藏在心里。"

**English:**

> "This is the last photo. After this, I stopped taking photos. Some things are best kept in the heart."

---

## 八、结局对话

### 8.1 普通结局

**场景：** 4-3
**触发条件：** 完成主线剧情

**像素照片下方文字：**

```
谢谢你的信任 — 外公
```

**English:**

```
Thank you for your trust — Grandpa
```

**外公画外音（见上文 2.3）。**

**Grandpa's voiceover (see section 2.3 above).**

**滚动字幕：**

```
x86 BIOS Simulator
                   普通结局

你修复了硬盘，恢复了外公留下的证据。
真相终于大白。

但你发现，还有很多事情可以做得更多……

所有软盘都收集了吗？
所有知识卡片都找到了吗？
暗室的门后面有什么？

也许，这不是结束。
```

**English:**

```
x86 BIOS Simulator
                   Normal Ending

You repaired the hard drive and recovered the evidence
Grandpa left behind. The truth finally came to light.

But you realize there's still more that could be done...

Have you collected all the floppy disks?
Have you found all the knowledge cards?
What's behind the door to the secret room?

Maybe this isn't the end.
```

---

### 8.2 隐藏结局

**场景：** 4-4
**触发条件：** 完成主线 + 收集所有软盘 + 找到所有密码

**暗室电脑屏幕文字（见上文 3.9）。**

**Secret room computer screen text (see section 3.9 above).**

**外公告别视频独白（见上文 2.4）。**

**Grandpa's farewell video monologue (see section 2.4 above).**

**结局画面文字：**

```
x86 BIOS Simulator
                   隐藏结局

你找到了暗室，看到了外公最后的留言。
他一直相信，你会找到真相。

证据已经完整，外公的心愿也完成了。

他留给你的，不只是证据。
是知识，是力量，是对你的信任。

"时间胶囊"项目最终成为了——
一个祖父留给孙辈的礼物。
```

**English:**

```
x86 BIOS Simulator
                   Hidden Ending

You found the secret room and saw Grandpa's final message.
He always believed you would find the truth.

The evidence is now complete, and Grandpa's wish is fulfilled.

What he left for you isn't just evidence.
It's knowledge, it's strength, and it's his faith in you.

The "Time Capsule" project ultimately became —
a gift from a grandfather to his grandchild.
```

---

### 8.3 彩蛋结局

**场景：** 4-5
**触发条件：** 在特定时间（1998-06-15）启动电脑

**DOS 屏幕显示（见上文 3.10）。**

**DOS screen display (see section 3.10 above).**

**雪花屏后显示：**

```
The end...

...or just the beginning?

外公的声音：

"你以为这就结束了？
 还有更多东西等着你去发现。"
```

**English:**

```
The end...

...or just the beginning?

Grandpa's voice:

"You think this is over?
 There's more out there waiting to be discovered."
```

**主菜单变化：**

```
[ 开始新游戏 ]
[ 继续游戏   ]
[ 硬核模式   ]
[ 沙盒模式   ]   ← 新解锁！
[ 设置       ]
[ 退出游戏   ]

★ 全成就解锁：时间胶囊 ★
"1998 年 6 月 15 日，一切开始的地方。"
```

**English:**

```
[ Start New Game     ]
[ Continue Game      ]
[ Hardcore Mode      ]
[ Sandbox Mode       ]   ← Newly unlocked!
[ Settings           ]
[ Exit Game          ]

★ All Achievements Unlocked: Time Capsule ★
"June 15, 1998 — where it all began."
```

---

### 8.4 坏结局（格式化 C 盘）

**场景：** 特殊触发（任意章节）
**触发条件：** 玩家在 DOS 中执行 `FORMAT C:` 命令

**过程：**

1. DOS 显示格式化确认提示：
   ```
   WARNING: ALL DATA ON NON-REMOVABLE DISK
   DRIVE C: WILL BE LOST!
   Proceed with Format (Y/N)?
   ```
2. 玩家输入 `Y` 后，屏幕显示格式化进程：
   ```
   Formatting C:  1% complete
   Formatting C:  2% complete
   ...
   ```
3. 在格式化过程中，屏幕上突然闪现出外公的影像——外公坐在电脑前的画面一闪而过，像是在看着玩家
4. 格式化完成，CRT 屏幕变黑
5. 所有收集品从物品栏中消失，笔记本内容变为空白

**外公画外音（最后的声音）：**

> ……
>
> 孩子，你删掉了它。
>
> 这些证据……我藏了二十多年的证据……
> 不管是为了正义还是为了什么，它们都不在了。
>
> 我不怪你。也许你不知道这意味着什么。
> 也许……我应该把真相直接告诉你，而不是藏在这些扇区里。
>
> 但那些已经不重要了。
>
> 记住一件事就好——
> 无论你用什么方式操作电脑，每一个命令都有后果。
> 有些删除，是没有办法恢复的。
>
> ……

**结局画面文字：**

```
x86 BIOS Simulator
                   坏结局：灰飞烟灭

你格式化了 C 盘。

外公花了二十年保存的证据，
你用一条命令就抹去了。

不要格式化 C 盘。
—— 外公最后的话，你没有听。
```

**English:**

```
x86 BIOS Simulator
                   Bad Ending: Gone to Ashes

You formatted the C drive.

The evidence Grandpa spent twenty years preserving,
you erased with a single command.

Do not format the C drive.
—— Grandpa's last words. You didn't listen.
```

---

## 九、玩家选择选项汇总

### 9.1 分支点 1：是否信任张阿姨（第二章开头）

**场景：** 第二章开头
**触发条件：** 第二次与张阿姨对话时

- **A.** "她只是个热心的邻居，应该没问题。" → 后续对话更加开放 / 成就"信任他人"
- **B.** "我需要谨慎，不能什么都告诉她。" → 后续对话更加保留 / 成就"谨慎行事" / 张阿姨会主动透露更多

**English:**

- **A.** "She's just a kind neighbor, she should be fine." → Subsequent dialogue is more open / Achievement "Trust in Others"
- **B.** "I need to be careful; I can't tell her everything." → Subsequent dialogue is more guarded / Achievement "Proceed with Caution" / Aunt Zhang will volunteer more information

---

### 9.2 分支点 2：接到第一次威胁电话后

**场景：** 第二章（接电话事件后）
**触发条件：** 接到李德胜第一次威胁电话后

- **A.** "外公把任务交给了我，我不能退缩。" → 笔记本记录："外公相信我。我也要相信自己。"
- **B.** "也许外公不想让我卷入危险的事……" → 笔记本记录："我有点害怕。但箭在弦上，不得不发。"
- **C.** "（假装没发生过）继续工作。" → 笔记本记录："……"

**English:**

- **A.** "Grandpa entrusted this mission to me. I can't back down." → Notebook record: "Grandpa believes in me. I need to believe in myself too."
- **B.** "Maybe Grandpa didn't want me involved in something dangerous..." → Notebook record: "I'm a little scared. But the arrow is already on the bowstring — there's no turning back."
- **C.** "(Pretend it never happened) Continue working." → Notebook record: "..."

---

### 9.3 分支点 3：发现完整证据后——如何处理

**场景：** 第三章结尾 / 第四章
**触发条件：** 解密完整证据后，第三次电话之前
**重要性：** 这是玩家情感投入最高的决策点。三个选项将导向截然不同的第四章体验和结局。

---

#### 选项 A：正义线——"把证据交给警方"

> "外公当年没做到的事，我来做。这些证据已经足够了。"

**后续影响：**

- 玩家将证据拷贝到 U 盘，通过邻居张阿姨的帮助联系了当年负责此案的退休老刑警
- 第四章变为"等待与保护"主题：玩家一边等待警方调查结果，一边发现阁楼有被入侵的迹象（门锁被撬、窗户玻璃有划痕）
- 李德胜的第三次电话变得更加疯狂："你以为交给警察就有用？那些人我都认识！"
- 最终结局：新闻报道"20 年旧案重启，李某某再次被捕"，但外公的暗室始终没有打开
- **结局画面：** 玩家坐在阁楼里，CRT 屏幕上播放着新闻，窗外是警车的灯光。外公的声音响起："孩子，你做到了。但我还有些话……没来得及说。"

**结局解锁：** "正义终将到来"成就
**代价：** 无法看到暗室中最后的告别视频

---

#### 选项 B：真相线——"先找到暗室"

> "外公把最重要的东西留在了暗室里。我要先完成他的遗愿。"

**后续影响：**

- 玩家选择暂时不报警，继续收集密码线索
- 第四章变为"与时间赛跑"主题：李德胜似乎察觉到了什么，电话越来越频繁，门外的异响也越来越近
- 张阿姨第三次来访时主动提供了关键线索："你外公说过，密码和你们的生日有关……"
- 最终打开暗室，看到外公最后的告别视频，获得完整证据链
- 最终结局：完整的证据交给了可靠的人，外公的遗愿彻底完成
- **结局画面：** 暗室中，外公的像素画像在屏幕上闪烁。外公说："孩子，你做到了。这些证据，还有这间暗室里的所有东西……都是留给你的。不是为了复仇，是为了让你记住——知识是最强大的武器。"

**结局解锁：** "时间胶囊"隐藏结局（唯一能看到完整告别视频的路线）
**代价：** 过程更加危险，李德胜的威胁持续升级

---

#### 选项 C：释然线——"也许该放手了"

> "这些证据已经过去了二十多年。外公把它藏起来，也许就是不想让我卷入危险……"

**后续影响：**

- 玩家将软盘和文件收好，放回抽屉，关闭了电脑
- 第四章变为"内心的挣扎"主题：玩家开始做其他事（整理房间、看书），但脑海中不断浮现那些证据
- 张阿姨来访时说了一段关键的话："你外公有一次喝醉了，跟我说：'张姐，人这辈子最怕的不是坏人逍遥法外，而是自己什么都不做。'"
- 半夜，玩家无法入睡，重新打开了电脑
- 最终，玩家还是选择了处理证据——但方式是匿名举报
- **结局画面：** 玩家站在邮筒前，手里拿着一个匿名信封。天刚亮，城市开始苏醒。外公的声音响起："不管你做了什么选择，你都长大了。我为你骄傲。"

**结局解锁：** "释然"成就
**特点：** 最安静但最有力量的结局，外公的画外音与其他结局不同，表达了对玩家选择的无条件接纳

---

**English:**

#### Option A: Justice — "Turn the evidence over to the police"

> "What Grandpa couldn't do back then, I'll do. This evidence is enough."

**Consequences:**
- The player copies the evidence to a USB drive and contacts a retired detective through Aunt Zhang
- Chapter 4 shifts to a "waiting and protection" theme: the player discovers signs of intrusion at the attic (jimmyed lock, scratched window)
- Li Desheng's third call becomes more frantic: "You think handing it to the police will help? I know all those people!"
- Ending: News report about the cold case being reopened, but the secret room is never opened
- **Ending scene:** Player sitting in the attic, news playing on the CRT, police lights outside the window

**Unlock:** "Justice Will Prevail" achievement
**Cost:** Cannot see the farewell video in the secret room

---

#### Option B: Truth — "Find the secret room first"

> "The most important thing Grandpa left is in the secret room. I need to fulfill his last wish first."

**Consequences:**
- Player chooses not to call police yet, continues collecting password clues
- Chapter 4 shifts to a "race against time" theme: Li Desheng's calls grow more frequent, strange sounds outside
- Aunt Zhang's third visit provides the key clue: "Your grandpa once said the password has something to do with your birthdays..."
- Opens the secret room, sees Grandpa's farewell video, obtains the complete evidence chain
- **Ending scene:** In the secret room, Grandpa's pixel portrait flickers on screen

**Unlock:** "Time Capsule" hidden ending (the only route to see the complete farewell video)
**Cost:** More dangerous process, escalating threats from Li Desheng

---

#### Option C: Letting Go — "Maybe I should let it go"

> "This evidence is over twenty years old. Grandpa hid it — maybe he didn't want me to get involved..."

**Consequences:**
- Player puts the disks and files back in the drawer, turns off the computer
- Chapter 4 shifts to an "inner struggle" theme: the player tries to do other things but can't stop thinking about the evidence
- Aunt Zhang visits and says something crucial: "Your grandpa once got drunk and told me: 'Zhang Jie, the worst thing in life isn't bad people getting away — it's doing nothing yourself.'"
- At midnight, unable to sleep, the player reopens the computer
- Ultimately, the player chooses to act — through anonymous reporting
- **Ending scene:** Player standing at a mailbox at dawn with an anonymous envelope

**Unlock:** "Letting Go" achievement
**Feature:** The quietest but most powerful ending — Grandpa's voiceover expresses unconditional acceptance of the player's choice

---

### 9.4 分支点 4：暗室门前——是否进入

**场景：** 第四章
**触发条件：** 到达暗室门前

- **A.** "（输入密码：你的生日 + 外公的生日）" → 密码正确：进入暗室，隐藏结局 / 密码错误：提示重试
- **B.** "也许我不应该进去……" → 提示："外公在这里等了你 20 多年" / 可稍后再回来
- **C.** "（查看外公留下的密码线索）" → 打开笔记本，显示之前收集的密码提示

**English:**

- **A.** "(Enter password: your birthday + Grandpa's birthday)" → Correct password: enter the secret room, hidden ending / Incorrect password: prompt to retry
- **B.** "Maybe I shouldn't go in..." → Prompt: "Grandpa has been waiting here for you for over 20 years" / Can come back later
- **C.** "(Check the password clues Grandpa left)" → Opens notebook, displays previously collected password hints

---

### 9.5 张阿姨第一次见面选择

**场景：** 第二章，场景 2-1 后
**触发条件：** 第一次与张阿姨对话

- **A.** "您认识我外公？" → 了解邻居关系和外公辞职后的状态
- **B.** "您知道外公以前在做什么吗？" → 了解辛巳游戏工作室和外公状态变化
- **C.** "（点头，没有说话）" → 张阿姨主动邀请

**English:**

- **A.** "You knew my grandpa?" → Learn about the neighbor relationship and Grandpa's state after quitting
- **B.** "Do you know what Grandpa used to do?" → Learn about Xinsi Game Studio and Grandpa's state changes
- **C.** "(Nodded, said nothing)" → Aunt Zhang voluntarily invites you over

---

### 9.6 张阿姨第二次拜访选择

**场景：** 第二章，场景 2-3 后
**触发条件：** 第二次与张阿姨对话

- **A.** "那个人长什么样？" → 获取李德胜外貌描述
- **B.** "后来呢？发生了什么？" → 了解后续发展
- **C.** "这个人是李德胜吗？" → 确认身份，张阿姨欲言又止

**English:**

- **A.** "What did that person look like?" → Get a description of Li Desheng's appearance
- **B.** "What happened after that?" → Learn about subsequent developments
- **C.** "Is this person Li Desheng?" → Confirm identity; Aunt Zhang stops herself mid-sentence

---

### 9.7 张阿姨第三次拜访选择

**场景：** 第三章，场景 3-4 后
**触发条件：** 第三次与张阿姨对话

- **A.** "我发现了外公当年调查的真相……" → 张阿姨透露外公的留言
- **B.** "您是不是还有什么没告诉我？" → 触发信封支线
- **C.** "外公有没有留信给您？" → 直接触发信封支线

**English:**

- **A.** "I discovered the truth about Grandpa's investigation..." → Aunt Zhang reveals Grandpa's message
- **B.** "Is there something you haven't told me?" → Triggers the envelope side quest
- **C.** "Did Grandpa leave a letter with you?" → Directly triggers the envelope side quest

---

### 9.8 李德胜第一次来电后选择

**场景：** 第二章
**触发条件：** 接到李德胜第一次威胁电话后

- **A.** "（挂断电话，心跳加速）"
- **B.** "（查看来电显示——没有号码）"
- **C.** "（记录下录音的内容到笔记本）"

**English:**

- **A.** "(Hangs up, heart racing)"
- **B.** "(Checks caller ID — no number)"
- **C.** "(Records the content of the recording in the notebook)"

---

---

## 十、自由交互系统 — 图灵完备式可玩性

> 本章定义游戏的自由交互层。玩家不再是"按步骤做题"，而是面对一台真实的模拟电脑，
> 用自由组合的命令、中断调用和调试工具来诊断问题、发现秘密、修复系统。
> **没有唯一解——只有你想不到的解法。**

---

### 10.1 核心理念

**从"教科书练习"到"真正的计算机取证"：**

| 传统设计 | 图灵完备设计 |
|----------|-------------|
| 教程说"输入 INT 13h 读扇区" | 教程说"证据藏在磁盘某处，你自己找" |
| 告诉玩家扇区号 | 玩家自己分析分区表推导扇区号 |
| 预设唯一修复步骤 | 提供工具，修复方式由玩家决定 |
| 每个谜题一个答案 | 玩家可以用命令组合创造新解法 |
| 技术知识是"考试内容" | 技术知识是"解决问题的工具" |

**设计原则：**

1. **可编程性** — 玩家可以编写批处理脚本和汇编代码来自动化任务
2. **可组合性** — 命令和工具可以自由组合，产生预期内但非预设的效果
3. **可逆性** — 除了 `FORMAT C:`，大多数操作可以撤回（通过备份/写回）
4. **可发现性** — 系统中存在大量主线之外的隐藏内容，等待有技术能力的玩家自行发掘
5. **涌现性** — BIOS 设置、磁盘数据、游戏状态三者联动，修改一个会影响其他

---

### 10.2 DEBUG 调试器 — 核心交互工具

**这是游戏的"瑞士军刀"。** DOS 的 DEBUG.COM 被完整模拟，玩家可以用它进行：

#### 10.2.1 十六进制查看与编辑

```
C:\>debug

# 查看 MBR（扇区 0）
-d 0 1ff

# 输出示例：
# 0B3C:0000  EB 3C 90 4D 53 44 4F 53-35 2E 30 00 02 08 01 00
# 0B3C:0010  02 00 02 00 00 F8 00 00-3F 00 FF 00 3F 00 00 00
# ...
# 0B3C:01BE  80 01 01 00 0B FE FF FF 3F 00 00 00 C1 07 3D 01  ← 分区表项 1
# 0B3C:01CE  00 00 00 00 00 00 00 00-00 00 00 00 00 00 00 00  ← 分区表项 2（空）
# 0B3C:01DE  00 00 00 00 00 00 00 00-00 00 00 00 00 00 00 00  ← 分区表项 3（空）
# 0B3C:01EE  00 00 00 00 00 00 00 00-00 00 00 00 00 00 55 AA  ← 签名

# 直接修改分区表（修复 MBR 的方式之一）
-e 01be 80 01 01 00 0b fe ff ff 3f 00 00 00 c1 07 3d 01

# 用 INT 13h 读取任意扇区到内存
# 先设置寄存器，然后执行中断
-r ax
:201        ← AH=02h (读), AL=01h (读 1 个扇区)
-r bx
:0200       ← 读到 ES:BX = 0000:0200
-r cx
:0001       ← CH=00h, CL=01h (扇区 1)
-r dx
:0080       ← DH=00h, DL=80h (硬盘)
-int 13
```

**剧情整合：** 外公的 DIARY.TXT 中写道："如果你看到了 DEBUG，说明你已经准备好用最底层的方式查看这台电脑了。记住——每个字节都有意义。"

---

#### 10.2.2 汇编器与反汇编器

玩家可以用 DEBUG 的 A 命令直接编写汇编代码：

```
C:\>debug

# 编写一个读取隐藏扇区的小程序
-a 0100
XXXX:0100 mov ah,02        ; 读扇区
XXXX:0102 mov al,01        ; 读 1 个扇区
XXXX:0104 mov ch,00        ; 柱面 0
XXXX:0106 mov cl,0a        ; 扇区 10（隐藏数据的起始位置）
XXXX:0108 mov dh,00        ; 磁头 0
XXXX:010A mov dl,80        ; 硬盘
XXXX:010C mov bx,0200      ; 缓冲区地址
XXXX:010F int 13           ; 调用 BIOS
XXXX:0111 int 3            ; 断点

# 执行
-g=0100

# 查看读到的数据
-d 0200 03ff
```

**图灵完备体现：** 玩家可以编写任意汇编程序来操作硬件。理论上，玩家可以在 BIOS 模拟器中运行任何实模式程序。游戏会引导，但不会限制。

---

#### 10.2.3 磁盘编辑（LBA 模式）

对于支持扩展 INT 13h 的模拟 BIOS，玩家可以用 LBA 模式直接访问 28 位扇区地址：

```
# 扩展 INT 13h 读取 LBA 扇区
-r ax
:4201       ← AH=42h (扩展读), AL=01h
-r ds
:0000
-r si
:0210       ← DS:SI 指向 DAP (Disk Address Packet)

# DAP 结构（写入内存 0210 处）
-e 0210
10 00 00 00 01 00 00 00 00 08 30 00
#    |        |        |
#  包大小   扇区数   LBA 低4字节
#  (16B)   (1个)   (0x00300800 = 扇区 3147776)

-r dx
:0080       ← 硬盘
-int 13

-d 0800 09ff
```

**剧情整合：** 在第三章中，外公的工作日志写道："我用 LBA 模式把证据藏在了扇区 0x00300800。这个地址是 300 万 × 1.049 的十六进制表示——代表那笔被贪污的 300 万美元。"

玩家必须理解 LBA 地址的含义，才能找到隐藏的证据。

---

### 10.3 DOS 命令系统 — 可编程环境

#### 10.3.1 完整支持的命令

| 命令 | 用途 | 剧情整合 |
|------|------|----------|
| `dir` | 列目录 | 发现文件和线索 |
| `type <file>` | 查看文件内容 | 阅读信件、日记、证据 |
| `cd <dir>` | 切换目录 | 探索不同位置 |
| `copy <src> <dst>` | 复制文件 | 备份证据、创建副本 |
| `del <file>` | 删除文件 | 可以删除非关键文件，关键文件无法删除 |
| `ren <old> <new>` | 重命名 | 修改文件以解密 |
| `format c:` | 格式化 C 盘 | **坏结局** |
| `fdisk` | 分区管理 | 分析/修复分区表 |
| `debug` | 调试器 | **核心工具**（见 10.2） |
| `edit` | 文本编辑器 | 修改配置文件、创建批处理 |
| `debug /b` | 批处理调试模式 | 逐行执行汇编 |
| `chkdsk` | 检查磁盘 | 发现损坏的扇区 |
| `debug -h` | DEBUG 帮助 | 提示用法 |

#### 10.3.2 批处理文件编程

玩家可以创建 `.BAT` 文件来自动化任务：

**示例1：自动备份所有软盘内容到硬盘**

```bat
@echo off
echo Backing up all floppy disks...

a:
copy *.txt c:\backup\a_disk\
copy *.dat c:\backup\a_disk\

echo Backup complete.
echo Remember: do not format C drive.
```

**示例2：批量读取硬盘扇区并保存**

```bat
@echo off
rem This batch file uses DEBUG to read sectors 198-202
rem and saves them to a file for analysis

debug < c:\scripts\read_sectors.txt
```

对应的 `read_sectors.txt`（DEBUG 的批处理输入）：

```
d 0 1ff
d 200 3ff
d 400 5ff
d 600 7ff
d 800 9ff
q
```

**示例3：自动化 MBR 修复流程**

```bat
@echo off
echo MBR Repair Tool v1.0
echo ===================
echo.
echo Reading current MBR...
debug < c:\scripts\dump_mbr.txt
echo.
echo Checking MBR signature...
rem Check if bytes 0x1FE-0x1FF are 55 AA
debug -e 01fe 55 aa
echo.
echo Writing repaired MBR to disk...
debug < c:\scripts\write_mbr.txt
echo.
echo Repair complete. Reboot to verify.
```

**图灵完备体现：** 玩家可以用批处理+DEBUG 实现任意磁盘操作流程。游戏不预设唯一的修复方案——玩家可以自己设计修复策略。

---

### 10.4 法医取证小游戏 — 自由探索模式

**触发条件：** 第二章，获得 DISK_02 后解锁

#### 10.4.1 设计理念

玩家获得一个"取证工具包"——其实就是对 DOS 命令的系统性引导。游戏不再手把手教"下一步做什么"，而是提出问题，让玩家自己寻找答案。

#### 10.4.2 取证任务系统

**任务 1：磁盘健康检查**

游戏提示：
> "这台电脑的硬盘似乎有问题。检查一下磁盘的状态。"

玩家需要自己想到：
```
C:\>chkdsk c:
```

输出可能是：
```
Volume SYSTEM created 06-15-1998
Volume Serial Number is 1E5F-3A2B

  1,258,291,200 bytes total disk space
      524,288 bytes in 2 hidden files
    2,097,152 bytes in 45 directories
  524,288,000 bytes in 234 user files
  731,379,712 bytes available on disk

      4,096 bytes in each allocation unit
    307,200 total allocation units on disk
    178,559 allocation units available on disk

  655,360 total bytes memory
  589,824 bytes free
```

玩家需要注意到：**2 个隐藏文件**。但 `dir` 命令看不到它们。如何查看？

（引导玩家使用 DEBUG 查看扇区）

---

**任务 2：分析分区表**

游戏提示：
> "硬盘有 8GB，但 C 盘只有 2GB。剩下的空间去哪了？"

玩家需要：
```
C:\>debug
-d 0 1ff          ← 读取 MBR
```

然后分析偏移 `0x1BE` 到 `0x1FD` 的四个分区表项，发现：
- 分区 1：类型 0Bh (FAT32)，2GB，C 盘
- 分区 2：类型 FFh（自定义！），6GB，**隐藏**
- 分区 3、4：空

玩家需要理解类型 `FFh` 是什么，以及如何访问这个隐藏分区。

---

**任务 3：恢复损坏的扇区**

游戏提示：
> "硬盘的第 200 号扇区似乎被覆盖了。修复它。"

**随机化设计：** 每次游戏，被损坏的扇区编号和损坏内容都不同。

**多种修复方案（玩家可任选）：**

**方案 A：从备份扇区恢复**
```
# 用 DEBUG 读取备份扇区（扇区 210）的内容，写入扇区 200
C:\>debug
# 设置 INT 13h 参数读取备份扇区
-r ax
:201
-r bx
:0200
-r cx
:00d2        ← 扇区 210 = CHS(0,0,210) ... 需要换算
-int 13
# 现在数据在 0200 处，写回扇区 200
-r ax
:0301        ← AH=03h 写扇区
-r cx
:00c8        ← 扇区 200
-int 13
```

**方案 B：手动重建数据**
```
# 查看扇区 199 和 201，推断扇区 200 应该是什么内容
C:\>debug
-d 0 1ff           ← 扇区 199 的内容
# ... 分析数据模式 ...
# 用 -e 命令直接在 0200 处写入修复后的数据
-e 0200 4D 5A 90 ...
# 写入扇区 200
-r ax
:0301
-r cx
:00c8
-int 13
```

**方案 C：使用 INT 13h 批量重写**
```
# 编写一个汇编小程序自动恢复多个损坏扇区
C:\>debug
-a 0100
mov ah,02
mov al,01
mov ch,00
mov cl,01
mov dh,00
mov dl,80
mov bx,0200
int 13
# ... 更多代码 ...
-g=0100
```

---

**任务 4：解密隐藏消息**

游戏提示：
> "隐藏分区里有一个加密的文件。密码藏在别处。"

玩家需要：
1. 用 DEBUG 读取隐藏分区的文件头
2. 发现文件被 XOR 加密（签名分析）
3. 在其他文件/录音带/照片中找到密钥
4. 编写解密程序（汇编或批处理）

```
# XOR 解密示例
C:\>debug encrypted.dat
-e 0200           ← 加密数据已在内存中
# 逐字节 XOR 0x42（密钥来自录音带 #5 的日期 6/4/2 → 642 → 0x42）
# 或者编写自动化解密程序：
-a 0300
mov cx,0100       ← 256 字节
mov si,0200       ← 数据地址
mov bl,42         ← 密钥
loop_start:
xor [si],bl
inc si
loop loop_start
int 3
-g=0300
# 查看解密后的数据
-d 0200 02ff
```

---

**任务 5：数据擦除与恢复（反取证挑战）**

游戏提示：
> "有人试图擦除硬盘上的数据。你能恢复吗？"

玩家需要理解：
1. `DEL` 命令只是标记文件目录项为"已删除"，数据还在
2. `FORMAT` 命令会清除 FAT 表，但数据区可能还在
3. 使用 DEBUG 直接扫描数据区，寻找文件签名（Magic Bytes）

```
# 扫描数据区寻找 PNG 文件签名
C:\>debug
-a 0100
mov ax,0201       ← 读 1 个扇区
mov bx,0200
mov cx,0001       ← 从扇区 1 开始
mov dx,0080
int 13

# 检查是否是 PNG（89 50 4E 47 0D 0A 1A 0A）
cmp byte [0200],89
jne next_sector
cmp byte [0201],50
jne next_sector
# ... 找到了！
```

---

**任务 6：TF 卡热插拔恢复 — 扇区归零修复（真实取证场景）**

**触发条件：** 第三章，地下室场景解锁后

**剧情背景：**

> 地下室的老书架上，有一张灰尘覆盖的 TF 卡（MicroSD）。
> 标签上手写："备份 — 1998 年 6 月 15 日"。
>
> 这是外公在最后一天做的数据备份——把关键证据额外拷贝到了这张卡上。
>
> 但 TF 卡似乎有物理损伤。插入读卡器后，DOS 显示：
>
> ```
> Invalid media type
> ```
>
> 用 DEBUG 查看扇区 0：
>
> ```
> C:\>debug
> -d 0 1ff
> 0B3C:0000  00 00 00 00 00 00 00 00-00 00 00 00 00 00 00 00
> 0B3C:0010  00 00 00 00 00 00 00 00-00 00 00 00 00 00 00 00
> ...
> （全零！MBR 被清空了！）
> ```
>
> 但外公的声音在旁边响起（录音带的额外内容）：
>
> "这张卡是热插拔弄坏的。当年我拔卡的时候没有先卸载，
> 文件系统的头部被清零了。但数据还在——
> 它在扇区后面。你只需要找到它。"

**玩家面临的挑战：**

前部扇区（0-N）全部为零——MBR、分区表、FAT 表、根目录全部丢失。
但用 DEBUG 往后翻，数据扇区仍然完好：

```
C:\>debug

# 扇区 0-100 全是零...

# 跳到扇区 500 看看
-r ax
:201
-r bx
:0200
-r cx
:01F4        ← 扇区 500（十进制）
-int 13
-d 0200 03ff

# 有数据了！可以看到：
# 0200: 4D 5A 90 00 03... ← MZ 头！这是 DOS 可执行文件
# 0210: 54 68 69 73 20... ← ASCII: "This..."
# 0220: ...
```

**关键认知：** 数据还在，只是"入口"（分区表、目录）丢失了。玩家需要：
1. 扫描磁盘，找到有效数据的起始扇区
2. 识别文件类型（Magic Bytes / 文件签名）
3. 重建文件的目录项，或者直接从扇区提取数据

**三种修复方案：**

**方案 A：直接提取（简单但只能恢复部分数据）**

```
# 手动找到数据开始的扇区，用 DEBUG 读出并保存
C:\>debug
# 扫描模式：读每个扇区，检查前两个字节
-a 0100
mov ax,0201     ; 读 1 扇区
mov bx,0400     ; 缓冲区
mov cx,0001     ; 从扇区 1 开始扫描
scan_loop:
push cx
mov dx,0080     ; 硬盘
int 13
jc error
cmp word [0400],0000  ; 检查是否全零
jne found_data
pop cx
inc cx          ; 下一个扇区
cmp cx,1000     ; 最多扫 1000 个扇区
jl scan_loop
int 3

found_data:
# CX 就是数据起始扇区号！
# 读出完整数据
mov ax,0220     ; 读 32 扇区（16KB）
mov bx,0800
int 13
# 现在数据在内存中，可以分析了
-d 0800 0fff
int 3

error:
int 3
-g=0100
```

**方案 B：文件签名扫描（最完整）**

```
# 扫描整个磁盘，标记所有文件头的位置
# 常见文件签名：
# 4D 5A (MZ)     — DOS 可执行文件
# 50 4B (PK)     — ZIP 压缩包
# 89 50 4E 47    — PNG 图片
# FF D8 FF       — JPEG 图片
# 52 61 72 21    — RAR 压缩包
# 23 21          — Shell 脚本/批处理

# 玩家可以编写一个批处理 + DEBUG 脚本来自动完成这个过程
```

对应的批处理自动化脚本：

```bat
@echo off
echo TF Card Recovery Tool v1.0
echo ==========================
echo Scanning for file signatures...
echo.
rem 用 DEBUG 逐扇区扫描并记录结果
debug /b < c:\tools\scan_signature.txt > c:\recovery\scan_log.txt
echo.
echo Scan complete. Results saved to C:\RECOVERY\SCAN_LOG.TXT
echo Use TYPE to view the log and identify files.
```

**方案 C：重建分区表（最优雅）**

如果玩家能够推断出原始分区的参数（起始扇区、大小、文件系统类型），可以直接用 DEBUG 重写 MBR：

```
C:\>debug
# 扇区 0 已经被读入内存
# 手动写入分区表项

# 分区 1：FAT12，起始扇区 2048，大小 2048 扇区
# 在偏移 0x1BE 处写入分区表项
-e 01be 80 01 01 00 01 00 01 00 00 08 00 00 00 08 00 00
#     |引导|CHS起始|类型|CHS结束|     LBA起始    |    大小    |

# 写入 MBR 签名
-e 01fe 55 aa

# 用 INT 13h 写回扇区 0
-r ax
:0301
-r bx
:0000
-r cx
:0001
-r dx
:0080
-int 13
```

**成功恢复后：**

DOS 现在可以识别 TF 卡，`dir` 显示：

```
C:\>a:
A:\>dir
 Volume in drive A is BACKUP
 Directory of A:\

BACKUP   TXT       2048  06-15-98
EVIDENCE DAT     153600  06-15-98
FINAL    TXT       8192  06-15-98
RECOVERY LOG       4096  06-15-98
         4 File(s)     167,936 bytes free
```

`RECOVERY.LOG` 是外公留下的记录：

> 如果你正在恢复这张卡，说明你也遇到了和我一样的问题——
> 热插拔导致文件系统头部损坏。
>
> 别担心。数据是不会骗人的。
> 它就在那里，只是你还没找到入口。
>
> 记住：文件系统的本质是索引。
> 没有索引，你还可以用内容来定位数据。
> 每个文件都有独特的签名——
> 4D 5A 是程序，50 4B 是压缩包，
> 89 50 4E 47 是 PNG 图片。
>
> 一个一个找，总会找全的。
>
> —— 外公

**这个任务的教学意义：**

1. **文件系统的本质** — FAT 是索引，不是数据本身。索引丢了，数据还在。
2. **文件签名** — 每种文件格式有独特的 Magic Bytes
3. **数据恢复流程** — 扫描 → 识别 → 提取 → 验证
4. **真实世界应用** — TF 卡/U 盘热插拔是日常风险，这个场景直接对应真实的数据恢复案例

---

#### 10.4.3 取证工具清单

完成取证任务后，玩家的"取证工具包"逐渐丰富：

| 工具 | 获取方式 | 用途 |
|------|----------|------|
| DEBUG 汇编器 | 第二章初始 | 底层磁盘操作 |
| CHS/LBA 计算器 | 学习卡片解锁 | 地址换算 |
| 文件签名表 | 任务 4 奖励 | 识别文件类型（Magic Bytes） |
| XOR 密钥检测器 | 任务 5 奖励 | 自动尝试常见 XOR 密钥 |
| 扇区比较工具 | 完成 3 个任务 | 对比两个扇区的差异 |
| 自动扫描脚本 | 任务 5 奖励 | 批量扫描隐藏数据 |
| 签名扫描器 | 任务 6 奖励 | 扫描全盘文件头并生成索引 |
| 分区表编辑器 | 任务 6 奖励 | 可视化编辑 MBR 分区表 |
| 零扇区检测器 | 任务 6 奖励 | 标记磁盘中被清零的区域 |
| 文件提取器 | 任务 6 奖励 | 从裸数据扇区中提取完整文件 |

---

### 10.5 秘密扇区探索系统 — 主线之外的隐藏世界

**设计目的：** 满足技术型玩家的探索欲，提供主线之外的额外内容。

#### 10.5.1 隐藏数据的层次

硬盘被设计为三层数据结构：

| 层级 | 位置 | 内容 | 发现难度 |
|------|------|------|----------|
| 表层 | C 盘正常文件 | README.TXT、游戏文件 | 主线剧情自动发现 |
| 中层 | 隐藏分区（分区类型 FFh） | EVIDENCE 文件、加密数据 | 需要分析分区表 |
| 深层 | 隐藏扇区（未分区空间） | 额外的外公录音、彩蛋、开发日志 | 需要直接用 INT 13h 扫描 |

#### 10.5.2 秘密扇区内容列表

玩家如果用 INT 13h 扫描主线未涉及的扇区，可以发现：

**扇区 #999（LBA 999）— 外公的第一段录音（原始版）**

*（与游戏中的录音带 #1 内容不同，是更原始、更私人的版本）*

> "今天是 1986 年 3 月 1 日。我录下这段话，不是给任何人听的——
> 只是想记录一下我此刻的心情。
>
> 我拿到了辛巳游戏工作室的 offer。李德胜亲自面试了我。
> 他说：'王志远，你是我见过的最懂底层的人。'
>
> 底层。这个词让我很舒服。
> 说实话，我不会社交，不会拍马屁，不会做 PPT。
> 但给我一台电脑，一块硬盘，我能告诉你每一个字节的故事。
>
> 也许这就是我的价值。"

---

**扇区 #1000 — 外公和外婆的结婚照数据（损坏的 BMP 头）**

用 INT 10h 切换到图形模式可以显示。是一张扫描质量很差的老照片——外公和外婆的结婚照，背景是简陋的照相馆布景。

如果此时播放录音带 #3（外公的三十周年纪念照独白），会触发额外的画外音：

> "这张照片……我找了好久。你外婆走后，我把所有她的照片都收起来了。
> 不是因为不想看——是因为太想看了。
>
> 有些记忆，放在心里比放在眼前更安全。"

---

**扇区 #1500 — "时间胶囊"项目的原始提案（英文）**

```
PROJECT CAPSULE — Original Proposal
Submitted by: Wang Zhiyuan
Date: January 15, 1986

Objective: Preserve critical human knowledge at the most
fundamental level — directly in disk sectors, bypassing
all software layers.

Rationale: Modern storage formats are fragile. A single
OS upgrade can render years of data inaccessible.
By storing data at the sector level, we ensure that
any machine capable of reading raw disk can recover
our knowledge.

Method: Use BIOS INT 13h to directly access and verify
sector-level data integrity. Encode redundancy through
distributed sector mapping...

[Document continues for 47 more sectors...]
```

**剧情意义：** 这份提案是外公亲自撰写的，证明"时间胶囊"最初是一个纯粹出于理想主义的技术项目，与后来的贪污完全无关。

---

**扇区 #2000-2047 — 外公的加密练习日志**

外公在隐藏证据前，自己练习了多种加密方法。这些扇区包含了他手写的加密/解密代码和笔记：

> XOR test #1: Key = 0x00 → no encryption
> XOR test #2: Key = 0xFF → simple inversion
> XOR test #3: Key = 0x55 → alternating pattern
> XOR test #4: Key = 0xAA → alternating pattern (inverted)
> XOR test #5: Key = date-derived → best choice
>
> Note: Don't use symmetric keys for the real data.
> I'll use a multi-layer approach:
> Layer 1: XOR with date-derived key
> Layer 2: Byte rotation by position
> Layer 3: Scramble sector order
>
> Only someone who understands ALL THREE layers
> can reconstruct the evidence. And only someone
> who knows where to look will even find the
> encrypted sectors.

**技术教学意义：** 这段内容实际上在教玩家后续解密的关键信息——加密是分层的，解密需要理解每一层。

---

**隐藏扇区 #3000 — 李德胜的忏悔信（从未寄出）**

*（这封信出现在隐藏扇区中，暗示外公不仅收集了犯罪证据，还保留了李德胜人性的一面。）*

> 志远：
>
> 写这封信的时候，我已经在看守所里了。
> 律师说我可能要判十年。我认了。
>
> 我知道你恨我。你有理由恨我。
> 我辜负了你的信任，辜负了所有人的信任。
>
> 但有一件事我想让你知道——
> "时间胶囊"最初的构想，我是真心的。
> 你说要"用最底层的方式保存人类知识"的时候，
> 我真的被打动了。
>
> 后来我变了。钱、权力、野心……
> 它们像病毒一样，一个字节一个字节地
> 覆盖了我的初心。
>
> 也许你永远不会看到这封信。
> 但如果你看到了——
>
> 对不起。
>
> 李德胜
> 1999 年 2 月 14 日

**剧情意义：** 这封信不会在主线中出现。只有用 INT 13h 直接扫描隐藏扇区的玩家才能发现。它让反派不再是一个扁平的"坏人"，而是曾经有过理想、后来堕落的复杂人物。

**注意：** 这封信的存在不影响玩家对李德胜行为的判断——贪污和威胁是事实。但它让故事更有深度。

---

#### 10.5.3 隐藏扇区的发现引导

游戏不会直接告诉玩家"去扫描扇区 #999"。引导是隐性的：

**引导 1：外公的加密笔记**（扇区 #2000-2047）
- 在第三章解密证据时，玩家需要读取这些扇区来理解加密方式
- 读取后，玩家会意识到"外公在硬盘里放了不止证据"

**引导 2：CHKDSK 的"2 个隐藏文件"**
- 第二章的磁盘检查任务中，chkdsk 报告有隐藏文件
- 但用 `dir` 命令看不到，用分区表分析也找不到对应的文件
- 唯一的方式是直接用 INT 13h 扫描

**引导 3：墙上的日历**
- 日历上某些日期有红圈标记：3月1日（扇区999）、2月14日（扇区3000）
- 不标记扇区号——玩家需要自己发现日期和扇区号的关系

**引导 4：录音带 #6 的幕后花絮**
- 外公说："我把所有东西都留在这台电脑里了。
  每一个扇区都有意义。有一些是给警察看的，
  有一些是给未来的人看的——如果你能读懂它们的话。"

---

### 10.6 BIOS 设置 — 可编程的系统底层

#### 10.6.1 BIOS 参数与游戏世界联动

修改 BIOS 设置会影响游戏世界的可见内容：

| BIOS 设置 | 修改效果 | 游戏影响 |
|-----------|----------|----------|
| 系统时间 | 改变游戏内时段 | 触发不同的窗外景色和音效 |
| 启动顺序 | 决定从哪个设备启动 | 从软盘启动可以看到不同内容 |
| 硬盘模式 | LBA/CHS/大型 | 影响能否访问大容量硬盘的隐藏区域 |
| 密码设置 | 设置/清除开机密码 | 下次开机需要密码（可用于保护存档） |
| 病毒警告 | 开启/关闭 | 开启时写入启动扇区会触发警告 |
| 隐藏分区可见性 | 修改分区表 | 让隐藏分区在 DOS 中可见 |

#### 10.6.2 BIOS 时间彩蛋

如果玩家将 BIOS 时间改为特定日期：

| 日期 | 效果 |
|------|------|
| 1998-06-15 | 彩蛋结局触发条件 |
| 2000-01-01 | Y2K Bug 彩蛋：开机显示 "THE MILLENNIUM BUG IS REAL" 然后正常启动 |
| 玩家生日 | 屏幕上闪过一行字："外公记得你的生日。" |
| 外公生日 (1956-03-08) | 额外的外公录音触发 |

#### 10.6.3 BIOS 密码系统

玩家可以在 BIOS Setup 中设置开机密码。这不是剧情必需的——但它有两个功能：

1. **保护存档**：设置了密码的存档，需要输入密码才能继续
2. **隐藏内容**：设置特定密码（如"XINSI"）可以解锁一个特殊的 BIOS 菜单项——"管理员模式"，显示硬盘的完整扇区映射图

---

### 10.7 系统联动 — 蝴蝶效应

**核心设计：BIOS 设置 / 磁盘数据 / 游戏世界 三方联动。**

玩家在任何一层的修改，都会在其他层产生连锁反应。

#### 10.7.1 联动示例

**示例 1：修改 BIOS 时间 → 影响故事线**

玩家将 BIOS 时间改到 1998 年 6 月 14 日：
- 游戏世界进入"1998 年模式"
- 外公的声音变得更年轻
- 窗外的景色从现代城市变成 90 年代的住宅区
- 桌上多了一封外公还没来得及寄出的信

**示例 2：用 DEBUG 写入数据 → 触发新对话**

玩家用 DEBUG 在某个扇区写入特定数据（如 "HELLO GRANDPA" 的十六进制）：
- 下次与张阿姨对话时，她会说一句额外的话：
  "奇怪……今天早上我梦到你外公了。他说：'那个孩子终于开始用 DEBUG 了。'"

**示例 3：格式化隐藏分区 → 改变结局**

如果在获得完整证据之前格式化了隐藏分区：
- 不触发坏结局（因为外公的证据还有一份在纸面上——张阿姨的信封）
- 但隐藏结局无法达成（暗室中的电脑无法启动）
- 解锁一个特殊的"遗憾结局"：玩家只拿到了部分证据

**示例 4：修改 MBR 分区表 → 让隐藏分区可见**

如果玩家用 DEBUG 将分区表项 2 的类型从 FFh 改为 0Bh（FAT32）：
- DOS 启动后可以 `dir` 看到 D 盘
- 但 D 盘的内容是加密的——需要解密
- 张阿姨在下一次对话中会说："你外公说过，有些东西就算你看到了，也不代表你能看懂。"

---

### 10.8 难度系统 — 自适应挑战

#### 10.8.1 三种模式

| 模式 | 引导程度 | 目标玩家 |
|------|----------|----------|
| **普通模式** | 完整教程，卡住检测，三级提示 | 所有玩家 |
| **硬核模式** | 无教程，无提示，需要自己理解系统 | 技术爱好者 |
| **沙盒模式** | 解锁所有区域，自由探索 | 彩蛋结局后解锁 |

#### 10.8.2 硬核模式特殊玩法

在硬核模式下，游戏会移除所有引导，并增加以下挑战：

**无知识卡片** — 玩家需要自己知道什么是 INT 13h、MBR、FAT12

**无任务提示** — 没有"下一步该做什么"的指引

**证据碎片化** — 同样的证据被分散到更多扇区，需要玩家自己推导扇区号

**反取证** — 李德胜的人会主动尝试擦除数据，玩家需要先备份再解密

**时间压力** — 存在一个不可见的计时器，如果在游戏内 7 天（现实时间约 3 小时）内没有完成，外公的某些证据会被彻底覆写

#### 10.8.3 沙盒模式

在彩蛋结局后解锁。沙盒模式移除所有剧情限制：

- 自由访问所有扇区
- 可以编写任意汇编程序
- 有一个"自由模式"的硬盘镜像，包含额外的挑战关卡
- 可以用内置的"汇编挑战编辑器"创建和分享自定义关卡

---

### 10.9 成就系统（扩展版）

#### 10.9.1 技术成就

| 成就 | 条件 | 难度 |
|------|------|------|
| **汇编新手** | 第一次用 DEBUG 的 A 命令写汇编 | 普通 |
| **中断猎人** | 成功调用所有模拟的 BIOS 中断 | 困难 |
| **磁盘考古学家** | 扫描超过 100 个扇区 | 普通 |
| **密码破译者** | 手动解密 XOR 加密文件 | 困难 |
| **MBR 重建师** | 从零重建一个完整的 MBR | 困难 |
| **隐藏扇区发现者** | 找到所有 5 个秘密扇区 | 极难 |
| **512 字节大师** | 在一个引导扇区（512 字节）内写入能运行的程序 | 极难 |
| **反取证专家** | 在反取证挑战中恢复所有被擦除的数据 | 极难 |

#### 10.9.2 故事成就

| 成就 | 条件 |
|------|------|
| **外公的学生** | 通过外公留下的所有教学内容 |
| **张阿姨的信物** | 获得张阿姨保管的信封 |
| **时间胶囊** | 发现外公的原始提案（扇区 #1500） |
| **李德胜的另一面** | 发现隐藏的忏悔信（扇区 #3000） |
| **1998 年 3 月 1 日** | 发现外公第一段录音（扇区 #999） |
| **全知者** | 完成所有取证任务 + 发现所有秘密扇区 |

---

### 10.10 自由交互系统的可玩性总结

#### 10.10.1 为什么这是"图灵完备"的

| 特征 | 体现 |
|------|------|
| **条件分支** | 玩家可以编写带条件判断的汇编代码和批处理脚本 |
| **循环** | DEBUG 的汇编器支持 LOOP 指令，批处理支持 GOTO |
| **内存读写** | DEBUG 可以读写任意内存地址 |
| **磁盘读写** | INT 13h 支持任意扇区的读写 |
| **中断调用** | 玩家可以调用任何模拟的 BIOS 中断 |
| **代码即数据** | 玩家可以在磁盘上存储可执行代码，也可以把数据当作代码执行 |
| **自修改代码** | DEBUG 的 A 命令和 E 命令可以修改内存中的代码 |

**这意味着理论上，玩家可以在 BIOS 模拟器中实现任何可计算的功能。** 游戏不会限制玩家——它提供工具和环境，玩家自己决定用这些工具做什么。

#### 10.10.2 游戏体验的多层次设计

| 玩家类型 | 体验路径 | 核心乐趣 |
|----------|----------|----------|
| **新手玩家** | 跟随主线剧情，完成教程引导的步骤 | 学习新知识，感受故事 |
| **技术爱好者** | 尝试不同的命令组合，探索隐藏内容 | 技术发现的成就感 |
| **极客玩家** | 用汇编编写自定义工具，发现所有秘密扇区 | 真正的"取证模拟"体验 |
| **硬核挑战者** | 硬核模式下零引导通关 | 纯粹的技术实力验证 |

#### 10.10.3 重玩价值

由于以下设计，玩家每次游戏体验可能不同：

1. **损坏扇区随机化** — 每次游戏，被覆盖的扇区位置不同
2. **隐藏内容分层** — 第一次玩可能只发现 2-3 个秘密扇区
3. **多种修复方案** — 同一个谜题有多种解法
4. **分支结局** — 三个核心结局 + 坏结局 + 隐藏的遗憾结局
5. **BIOS 设置影响** — 不同的设置组合产生不同的游戏体验

---

*文档版本：v2.0*
*新增章节：第十章 — 自由交互系统*
*编写日期：2026-05-27*
