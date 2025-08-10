---@meta

---@class DaggerSpecification
---@field [1] string
---@field tag string?
---@field branch string?
---@field method "github"|"local"|string?
---@field dependencies DaggerSpec|DaggerSpec[]?

---@alias DaggerSpec DaggerSpecification|string|DaggerSpec[]

---@class Dagger
---@field add fun(x: DaggerSpec)
---@field register fun(x: DaggerSpec)
