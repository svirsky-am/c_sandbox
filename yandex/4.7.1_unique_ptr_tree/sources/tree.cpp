#include "tree.h"

int main() {
    // TreeNode root(1);
    TreeNode_NG root(1);

    auto left_son = root.AddChild(10);
    auto middle_son = root.AddChild(20);
    auto right_son = root.AddChild(30);

    left_son->AddChild(100);
    left_son->AddChild(200);

    root.Print();


}