#include <iostream>
#include <vector>

class TreeNode {
private:
    int value;
    TreeNode* root = nullptr;
    std::vector<TreeNode*> children;

public:
    TreeNode(int val): value(val) {
    }

    TreeNode(const TreeNode&) = delete;
    TreeNode& operator=(const TreeNode&) = delete;

    TreeNode* AddChild(int child_value) {
        auto node = new TreeNode(child_value);
        node->root = this;
        children.push_back(node);
        return node;
    }

    ~TreeNode() {
        for (TreeNode* child : children) {
            delete child;
        }
    }

    void Print(int depth = 0) const {
        for (int i = 0; i < depth; ++i) {
            std::cout << " ";
        }
        std::cout << "- " << value << "\n";
        for (const auto& child : children) {
            child->Print(depth + 1);
        }
    }
};


// TreeNode_NG will be using unique_ptr
#include <memory>
#include <string>

struct TreeNode_NG {
private:
    int value;
    TreeNode_NG* root = nullptr;
    std::vector<std::unique_ptr<TreeNode_NG>> children;

public:
    TreeNode_NG(int val): value(val) {
    }

    TreeNode_NG(const TreeNode_NG&) = delete;
    TreeNode_NG& operator=(const TreeNode_NG&) = delete;

    TreeNode_NG* AddChild(int child_value) {
        children.push_back(std::make_unique<TreeNode_NG>(child_value));
        children.back()->root = this;
        return children.back().get();
    }

    void Print(int depth = 0) const {
        for (int i = 0; i < depth; ++i) {
            std::cout << " ";
        }
        std::cout << "- " << value << "\n";
        for (const auto& child : children) {
            child->Print(depth + 1);
        }
    }
};