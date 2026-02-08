package utils

import "github.com/gin-gonic/gin"

func Success(c *gin.Context, code int, data interface{}) {
	c.JSON(code, gin.H{"data": data})
}

func Error(c *gin.Context, code int, message string) {
	c.JSON(code, gin.H{"error": message})
}
