# 一.实现功能
实现了函数sys_spawn(),sys_set_priority(),TCB的内容相对于上节又增加新字段pub priority:isize,然后由于进程管理方法的改变，进程相关数据结构发生改变，故修改了上节实现的多个函数的TCB获取的部分代码，将从TASK_MANAGER获取改成由current_task()函数从PROCESSOR处获取。
# 二.简答作业
1.不是，BIG_STRIDE 取得很大，发生了反转现象，p2.stride+pass=250+10=260,由于是8bit存储的，stride的MAX为255，故260实际为5，故还是p2执行
2.如果优先级>=2,pass<=BigStride/2,即pass最大为BigStride/2,假设优先级为无穷大，可以得到pass最小趋近于0，故在加pass后，变化最大的和变化最小的差值最大为BigStride/2,故TRIDE_MAX – STRIDE_MIN <= BigStride / 2
3.impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	if self.0<BigStride/2&&other.0>=BigStride/2{
		return Some(Order::Gretter);
	}else if other.0<BigStride/2&&self.0>=BigStride/2{
		return Some(Order::Less);
	}else{
		if self.0>other.0{
			return Some(Order::Gretter)
	    	 }else {return Some(Order::Less)}
	}
    }
}



# 三.荣誉准则

1.在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 

2.此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> [rCore-Tutorial-Guide-2024S 文档](https://learningos.cn/rCore-Tutorial-Guide-2024S/)

3.我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4.我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。