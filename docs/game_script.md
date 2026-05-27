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

*文档版本：v1.0*
*源文档：DIALOGUE_REFERENCE.md*
*编写日期：2026-05-27*
*中英双语完整版*
